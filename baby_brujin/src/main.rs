mod cli;
mod input;
mod commands;

use cli::{Cli, Commands};
use clap::Parser;
use input::read_input;

fn main() {
    let cli = Cli::parse();
    match cli.command {
        Commands::Shuffle { file } => {
            let content = read_input(file).unwrap();
            match commands::shuffle_text(&content) {
                Ok(result) => println!("{}", result),
                Err(e) => eprintln!("Error: {}", e),
            }
        },
        Commands::Slice { file, min_length, max_length } => {
            let content = read_input(file).unwrap();
            match commands::slice_text(&content, min_length, max_length) {
                Ok(fragments) => println!("{}", fragments.join("\n")),
                Err(e) => eprintln!("Error: {}", e),
            }
        },
    }
}
