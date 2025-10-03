use clap::{Parser, Subcommand};

#[derive(Parser)]
#[command(name = "todo")]
#[command(about = "A simple TUI-based todo CLI", version, author)]
pub struct Cli {
    /// Use local storage
    #[arg(long)]
    pub local: bool,

    #[command(subcommand)]
    pub command: Option<Commands>, // subcommands still possible
}

#[derive(Subcommand)]
pub enum Commands {
    // add subcommands later
}
