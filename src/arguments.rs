use clap::Parser;

/// Fetch Minecraft server banners
#[derive(Parser)]
pub struct Arguments {
    /// Minecraft server addresses
    #[arg(required = true)]
    pub servers: Vec<String>,

    /// Omit list of players
    #[arg(short, long)]
    pub no_players: bool,

    /// Print additional information
    #[arg(short, long)]
    pub verbose: bool,

    /// Print additional debug information
    #[arg(short, long)]
    pub debug: bool,

    /// Override host name sent to server
    #[arg(short = 'H', long)]
    pub host: Option<String>,

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
