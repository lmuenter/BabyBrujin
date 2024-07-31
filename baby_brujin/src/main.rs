mod cli;
mod shuffle;

use cli::{Cli, Commands};
use clap::Parser;

fn main() {
    let cli = Cli::parse();

    match cli.command {
        Commands::Shuffle { file } => {
            match shuffle::shuffle_text(&file) {
                Ok(shuffled) => println!("{}", shuffled),
                Err(e) => eprintln!("Error reading file: {}", e),
            }
        }
    }
}
