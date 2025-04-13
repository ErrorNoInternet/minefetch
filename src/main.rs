#![feature(let_chains)]

mod arguments;
mod formatting;
mod protocol;

use arguments::{Arguments, Command, PingCommand};
use base64::{Engine, prelude::BASE64_STANDARD};
use clap::Parser;
use color_eyre::eyre::{ContextCompat, Result};
use crossterm::{
    ExecutableCommand,
    cursor::{self, MoveRight, MoveTo},
    style::Stylize,
};
use formatting::{Pad, component, legacy, pad};
use image::ImageReader;
use protocol::ping_server;
use serde_json::Value;
use std::{
    io::{Cursor, stdout},
    net::Ipv4Addr,
    time::Duration,
};
use tokio::net::{UdpSocket, lookup_host};
use viuer::{Config, terminal_size};

#[tokio::main]
async fn main() -> Result<()> {
    color_eyre::install()?;

    let args = Arguments::parse();
    match args.command {
        Command::Lan { once } => lan(&args, once).await,
        Command::Ping(ref command) => ping(&args, command).await,
    }
}

async fn lan(args: &Arguments, once: bool) -> Result<()> {
    fn between<'a>(text: &'a str, start: &str, end: &str) -> Option<&'a str> {
        let start_index = text.find(start)? + start.len();
        let end_index = text[start_index..].find(end)? + start_index;
        Some(&text[start_index..end_index])
    }

    let address = Ipv4Addr::new(224, 0, 2, 60);
    let listener = UdpSocket::bind((address, 4445)).await?;
    listener.join_multicast_v4(address, Ipv4Addr::UNSPECIFIED)?;

    let mut buf = [0u8; 256];
    loop {
        let (read, addr) = listener.recv_from(&mut buf).await?;
        let data = String::from_utf8_lossy(&buf[..read]);
        if args.debug {
            println!("{addr}: {data}")
        };

        if let Some((motd, port)) =
            between(&data, "[MOTD]", "[/MOTD]").zip(between(&data, "[AD]", "[/AD]"))
        {
            println!(
                "{}\n{}",
                format!("{}:{}", addr.ip(), port).bold(),
                motd.grey()
            );

            if once {
                break Ok(());
            }
            if !args.no_space {
                println!()
            }
        }
    }
}

async fn ping(args: &Arguments, command: &PingCommand) -> Result<()> {
    for (i, server) in command.servers.iter().enumerate() {
        let (addr, host) = if server.contains(':') {
            let (host, port) = server
                .split_once(':')
                .context("failed to parse server address")?;
            let addr = lookup_host((host, port.parse()?))
                .await?
                .next()
                .context("failed to resolve server address")?;
            (addr, host)
        } else {
            let addr = lookup_host((server.to_owned(), 25565))
                .await?
                .next()
                .context("failed to resolve server address")?;
            (addr, server.as_str())
        };
        let (data, latency) =
            ping_server(addr, command.host.as_ref().map_or(host, |v| v), args.debug).await?;
        print_server(command, server, &data, latency)?;

        if !args.no_space && i < command.servers.len() - 1 {
            println!();
        }
    }

    Ok(())
}

fn print_icon(command: &PingCommand, data: &Value) -> Result<Option<((u16, u16), u16)>> {
    let mut saved_position = None;
    if command.icon_size > 0
        && let Some(favicon) = data
            .get("favicon")
            .and_then(|value| value.as_str())
            .and_then(|string| string.split_once(','))
    {
        let mut position = cursor::position()?;
        let space = terminal_size().1 - position.1 - 1;
        if space < command.icon_size {
            position.1 = position.1.saturating_sub(command.icon_size - space);
        }
        saved_position = Some((
            position,
            command.padding.unwrap_or(command.icon_size * 2 + 1),
        ));

        let image = ImageReader::new(Cursor::new(BASE64_STANDARD.decode(favicon.1)?))
            .with_guessed_format()?
            .decode()?;
        viuer::print(
            &image,
            &Config {
                absolute_offset: false,
                height: Some(u32::from(command.icon_size)),
                transparent: true,
                ..Config::default()
            },
        )?;
    }
    if let Some((position, _)) = saved_position {
        stdout().execute(MoveTo(position.0, position.1))?;
    }
    Ok(saved_position)
}

fn print_server(
    command: &PingCommand,
    server: &str,
    data: &Value,
    latency: Duration,
) -> Result<()> {
    let saved_position = print_icon(command, data)?;
    macro_rules! draw_line {
        ($($arg:tt)*) => {
            if let Some((_, column)) = saved_position {
                stdout().execute(MoveRight(column))?;
            }
            println!($($arg)*);
        };
    }

    let ms = latency.as_millis().to_string();
    let latency_line = format!(
        "{ms} ms {}",
        match latency.as_millis() {
            0..=150 => "█",
            151..=300 => "▆",
            301..=450 => "▄",
            451..=600 => "▂",
            601.. => "▁",
        }
        .green()
        .on_dark_grey()
    );
    draw_line!(
        "{}{}",
        pad(
            &server.bold(),
            server.len(),
            command.width * 3 / 4,
            Pad::Left
        ),
        pad(&latency_line, ms.len() + 5, command.width / 4, Pad::Right),
    );

    let version_line = format!(
        "{} ({})",
        data["version"]["name"]
            .as_str()
            .context("expected version.name to be a string")?,
        ("v".to_string() + &data["version"]["protocol"].to_string())
    );
    let players_line = format!(
        "{} {} {}",
        data["players"]["online"],
        "/".grey(),
        data["players"]["max"]
    );
    let players_line_len =
        data["players"]["online"].to_string().len() + data["players"]["max"].to_string().len() + 3;
    draw_line!(
        "{}{}",
        pad(
            &version_line,
            version_line.len(),
            command.width * 3 / 4,
            Pad::Left
        ),
        pad(
            &players_line,
            players_line_len,
            command.width / 4,
            Pad::Right
        )
    );

    let mut players_sample = None;
    if !command.no_players
        && let Some(sample) = data["players"]["sample"].as_array()
        && !sample.is_empty()
    {
        players_sample = Some(sample);
    }
    let mut lines_drawn = if players_sample.is_some() { 3 } else { 2 };
    let formatted = if let Ok(ref component) = serde_json::from_value(data["description"].clone()) {
        Some(component::format(command.width, component))
    } else {
        data["description"]
            .as_str()
            .map(|str| legacy::format(command.width, str))
    };
    if let Some(formatted) = formatted {
        for line in formatted.lines() {
            lines_drawn += 1;
            draw_line!("{line}");
        }
    }
    if players_sample.is_some() || saved_position.is_some() {
        for _ in lines_drawn..command.icon_size as usize {
            draw_line!();
        }
    }

    if let Some(sample) = players_sample {
        let mut names = Vec::new();
        let mut column = 0;
        for (len, name) in sample.iter().filter_map(|player| {
            player["name"]
                .as_str()
                .map(|name| (name.len(), legacy::format(usize::MAX, name)))
        }) {
            column += len;
            if column > command.width - 3 {
                names.push(String::from("..."));
                break;
            }
            names.push(name);
        }
        draw_line!("{}", names.join(", ").grey().italic());
    }

    if command.verbose {
        print_verbose(data);
    }

    Ok(())
}

fn print_verbose(data: &Value) {
    println!();

    println!(
        "{}{}",
        "Enforces secure chat: ".bold(),
        data["enforcesSecureChat"].as_bool().unwrap_or_default()
    );
    if let Some(value) = data["preventsChatReports"].as_bool() {
        println!("{}{}", "Prevents chat reports: ".bold(), value)
    }
    if let Some(sample) = data["players"]["sample"].as_array() {
        println!("{}", "Player list sample:".bold());
        for player in sample {
            println!("\t{} ({})", player["name"], player["id"]);
        }
    }
}
