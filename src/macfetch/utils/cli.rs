use clap::Parser;

use crate::macfetch::constants;

/// A macOS Neofetch alternative written in Rust
#[derive(Parser)]
#[command(name = "macfetch")]
#[command(version = constants::VERSION)]
#[command(about = "A macOS Neofetch alternative written in Rust", long_about = None)]
pub struct Cli {}

pub fn handle_cli_args() {
    // Parse CLI args - clap handles --help and --version automatically
    Cli::parse();
}
