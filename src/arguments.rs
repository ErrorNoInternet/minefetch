use clap::Parser;

/// Fetch Minecraft server banners
#[derive(Parser)]
pub struct Arguments {
    /// Minecraft server addresses
    #[arg(required = true)]
    pub servers: Vec<String>,

    /// Print additional debug information
    #[arg(short, long)]
    pub debug: bool,

    /// Amount of space after server icon
    #[arg(short, long)]
    pub padding: Option<u16>,

    /// Size of the server icon
    #[arg(short, long, default_value_t = 5)]
    pub icon_size: u16,
}
