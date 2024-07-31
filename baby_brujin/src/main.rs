mod cli;
mod shuffle;
mod slice;

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
        },
        Commands::Slice { file, min_length, max_length } => {
            match slice::slice_text(&file, min_length, max_length) {
                Ok(fragments) => {
                    for fragment in fragments {
                        println!("{}", fragment);
                    }
                },
                Err(e) => eprintln!("Error processing file: {}", e),
            }
        }
    }
}
