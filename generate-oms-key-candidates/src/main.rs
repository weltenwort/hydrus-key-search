use std::num::ParseIntError;

use crate::{key_space::SymmetricKeySpace, search_key_space::search_key_space};
use clap::{Parser, Subcommand};

mod key_space;
mod search_key_space;

fn main() {
    let cli = Cli::parse();

    let key_space = match &cli.command {
        Commands::Symmetric {
            first_key,
            last_key,
        } => SymmetricKeySpace {
            first: *first_key,
            last: *last_key,
        },
    };

    let result = search_key_space(&key_space);

    println!("{}", result);
}

#[derive(Parser)]
struct Cli {
    #[clap(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    Symmetric {
        #[clap(parse(try_from_str=parse_hex_u64))]
        first_key: u64,
        #[clap(parse(try_from_str=parse_hex_u64))]
        last_key: u64,
    },
}

fn parse_hex_u64(input: &str) -> Result<u64, ParseIntError> {
    u64::from_str_radix(input, 16)
}
