use clap::{Parser, Subcommand};


#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]
pub struct Cli {
    #[clap(subcommand)]
    pub command: Commands,
}

#[derive(Subcommand, Debug)]
pub enum Commands {
    /// Shuffles the text in the provided file
    Shuffle {
        #[clap(short, long)]
        file: String,
    },
}