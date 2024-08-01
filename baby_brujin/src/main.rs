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
            let content = read_input(file).expect("Failed to read input");
            match commands::shuffle_text(&content) {
                Ok(result) => println!("{}", result),
                Err(e) => eprintln!("Error: {}", e),
            }
        },
        Commands::Slice { file, min_length, max_length } => {
            let content = read_input(file).expect("Failed to read input");
            match commands::slice_text(&content, min_length, max_length) {
                Ok(fragments) => println!("{}", fragments.join("\n")),
                Err(e) => eprintln!("Error: {}", e),
            }
        },
        Commands::Duplicate { file, length_section_duplicated, replication_depth } => {
            let content = read_input(file).expect("Failed to read input");
            match commands::duplicate(&content, length_section_duplicated, replication_depth) {
                Ok(result) => println!("{}", result),
                Err(e) => eprintln!("Error: {}", e),
            }
        }
    }
}
