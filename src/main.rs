use clap::{Parser, Subcommand};

mod az_types;
mod converters;
mod errors;
mod filesystem;

use converters::{az_to_env, env_to_az};
use errors::ConversionError;
use filesystem::reader::get_file_text;

use crate::filesystem::writer::write_file;

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

fn compose_convert(
    converter: impl Fn(String) -> Result<String, ConversionError>,
) -> impl FnOnce(&str, &str) -> Result<(), ConversionError> {
    move |input, output| {
        let file_content = get_file_text(input)?;
        let output_content = converter(file_content)?;
        write_file(output, output_content)?;
        Ok(())
    }
}

fn get_result(cmd: &Commands) -> Result<(), ConversionError> {
    match cmd {
        Commands::AzToEnv { input, output } => compose_convert(az_to_env::convert)(input, output),
        Commands::EnvToAz { input, output } => compose_convert(env_to_az::convert)(input, output),
    }
}

fn main() {
    let args = Args::parse();

    let result = get_result(&args.cmd);

    match result {
        Ok(_) => println!("Done."),
        Err(e) => println!("Error: {}", e),
    };
}
