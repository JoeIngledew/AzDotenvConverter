use std::{
    fs::{self, File},
    io::{self, BufRead, Read},
    path::Path,
};

use clap::{Parser, Subcommand};

mod az_types;
mod converters;
mod errors;
use az_types::AzOutputItem;
use converters::{az_to_env, env_to_az};
use errors::ConversionError;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    #[command(subcommand)]
    cmd: Commands,
}

#[derive(Subcommand, Debug, Clone)]
enum Commands {
    AzToEnv { input: String, output: String },
    EnvToAz { input: String, output: String },
}

fn main() {
    let args = Args::parse();

    let result = match &args.cmd {
        Commands::AzToEnv { input, output } => az_to_env::convert(input, output),
        Commands::EnvToAz { input, output } => env_to_az::convert(input, output),
    };

    match result {
        Ok(_) => println!("Done."),
        Err(e) => println!("Error: {}", e),
    };
}
