use clap::{Args, Parser, Subcommand};

/// Ping Minecraft servers from your terminal
#[derive(Parser)]
#[command(version)]
pub struct Arguments {
    #[command(subcommand)]
    pub command: Command,

    /// Print additional debug information
    #[arg(short, long)]
    pub debug: bool,

    /// Omit the space between each server
    #[arg(short, long)]
    pub no_space: bool,
}

#[derive(Subcommand)]
pub enum Command {
    /// Listen for servers opened to LAN
    Lan {
        /// Receive one server and exit
        #[arg(short, long)]
        once: bool,
    },

    Ping(PingCommand),
}

/// Ping a list of Minercraft servers
#[derive(Args)]
pub struct PingCommand {
    /// Minecraft server addresses
    #[arg(required = true)]
    pub servers: Vec<String>,

    /// Omit list of players
    #[arg(short, long)]
    pub no_players: bool,

    /// Override host names sent to server
    #[arg(short = 'H', long)]
    pub hosts: Vec<String>,

    /// Print additional information
    #[arg(short, long)]
    pub verbose: bool,

    /// Maximum width of all lines
    #[arg(short, long, default_value_t = 60)]
    pub width: usize,

    /// Amount of space after server icon
    #[arg(short, long)]
    pub padding: Option<u16>,

    /// Size of the server icon
    #[arg(short, long, default_value_t = 5)]
    pub icon_size: u16,
}
