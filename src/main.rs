#![feature(let_chains)]

mod arguments;
mod formatting;
mod protocol;

use arguments::Arguments;
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
use protocol::fetch_server;
use serde_json::Value;
use std::{
    io::{Cursor, stdout},
    time::Duration,
};
use tokio::net::lookup_host;
use viuer::{Config, terminal_size};

#[tokio::main]
async fn main() -> Result<()> {
    color_eyre::install()?;
    let args = Arguments::parse();

    for server in &args.servers {
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
            fetch_server(addr, args.host.as_ref().map_or(host, |v| v), args.debug).await?;
        print_server(&args, server, &data, latency)?;
    }

    Ok(())
}

fn print_icon(args: &Arguments, data: &Value) -> Result<Option<((u16, u16), u16)>> {
    let mut saved_position = None;
    if args.icon_size > 0
        && let Some(favicon) = data
            .get("favicon")
            .and_then(|value| value.as_str())
            .and_then(|string| string.split_once(','))
    {
        let mut position = cursor::position()?;
        let space = terminal_size().1 - position.1 - 1;
        if space < args.icon_size {
            position.1 = position.1.saturating_sub(args.icon_size - space);
        }
        saved_position = Some((position, args.padding.unwrap_or(args.icon_size * 2 + 1)));

        let image = ImageReader::new(Cursor::new(BASE64_STANDARD.decode(favicon.1)?))
            .with_guessed_format()?
            .decode()?;
        viuer::print(
            &image,
            &Config {
                absolute_offset: false,
                height: Some(u32::from(args.icon_size)),
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

fn print_server(args: &Arguments, server: &str, data: &Value, latency: Duration) -> Result<()> {
    let saved_position = print_icon(args, data)?;
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
        pad(&server.bold(), server.len(), args.width * 3 / 4, Pad::Left),
        pad(&latency_line, ms.len() + 5, args.width / 4, Pad::Right),
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
            args.width * 3 / 4,
            Pad::Left
        ),
        pad(&players_line, players_line_len, args.width / 4, Pad::Right)
    );

    let mut players_sample = None;
    if !args.no_players
        && let Some(sample) = data["players"]["sample"].as_array()
        && !sample.is_empty()
    {
        players_sample = Some(sample);
    }
    let mut lines_drawn = if players_sample.is_some() { 3 } else { 2 };
    let formatted = if let Ok(ref component) = serde_json::from_value(data["description"].clone()) {
        Some(component::format(args.width, component))
    } else {
        data["description"]
            .as_str()
            .map(|str| legacy::format(args.width, str))
    };
    if let Some(formatted) = formatted {
        for line in formatted.lines() {
            lines_drawn += 1;
            draw_line!("{line}");
        }
    }
    if players_sample.is_some() || saved_position.is_some() {
        for _ in lines_drawn..args.icon_size as usize {
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
            if column > args.width - 3 {
                names.push(String::from("..."));
                break;
            }
            names.push(name);
        }
        draw_line!("{}", names.join(", ").grey().italic());
    }

    if args.verbose {
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

    if let Some(sample) = data["players"]["sample"].as_array() {
        println!("{}", "Player list sample:".bold());
        for player in sample {
            println!("\t{} ({})", player["name"], player["id"]);
        }
    }
}
