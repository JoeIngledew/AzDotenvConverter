use std::{
    fs::{self, File},
    io::{self, BufRead, Read},
    path::Path,
};

use clap::{Parser, Subcommand};
use serde::{Deserialize, Serialize};

mod errors;
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

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
struct AzOutputItem {
    name: String,
    value: String,
    slot_setting: bool,
}

impl AzOutputItem {
    fn new(name: &str, value: &str) -> Self {
        AzOutputItem {
            name: String::from(name),
            value: String::from(value),
            slot_setting: false,
        }
    }
}

impl TryFrom<String> for AzOutputItem {
    type Error = ConversionError;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        let equals_ix = value
            .chars()
            .position(|c| c == '=')
            .ok_or(String::from("Invalid line"))?;
        let (name, value) = value.split_at(equals_ix);
        let value_without_equals: String = value.chars().skip(1).collect();
        Ok(AzOutputItem::new(name, &value_without_equals))
    }
}

fn read_lines<P>(filename: &P) -> io::Result<io::Lines<io::BufReader<File>>>
where
    P: AsRef<Path>,
{
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}

fn env_to_az(input: &str, output: &str) -> Result<(), ConversionError> {
    let input_lines = read_lines(&String::from(input))?;
    let mut output_items: Vec<AzOutputItem> = Vec::new();
    let mut errors: Vec<usize> = Vec::new();
    let mut ix: usize = 0;
    for line in input_lines.map_while(Result::ok) {
        if let Ok(output_item) = AzOutputItem::try_from(line) {
            output_items.push(output_item);
            ix += 1;
        } else {
            errors.push(ix);
        }
        ix += 1;
    }

    if !errors.is_empty() {
        return Err(ConversionError::Generic(format!(
            "Failed to parse, errors on lines: {:?}",
            errors
        )));
    }

    let new_file_text = serde_json::to_string(&output_items)?;
    fs::write(output, new_file_text)?;
    Ok(())
}

fn az_to_env(input: &str, output: &str) -> Result<(), ConversionError> {
    let path = Path::new(input);
    let mut file = File::open(path)?;
    let mut file_contents = String::new();
    file.read_to_string(&mut file_contents)?;

    let items: Vec<AzOutputItem> = serde_json::from_str(&file_contents)?;
    let output_items: Vec<String> = items
        .iter()
        .map(|x| format!("{}={}", x.name, x.value))
        .collect();

    let output_path = Path::new(output);

    let contents = output_items.join("\r\n");
    fs::write(output_path, contents)?;

    Ok(())
}

fn main() {
    let args = Args::parse();

    let result = match &args.cmd {
        Commands::AzToEnv { input, output } => az_to_env(input, output),
        Commands::EnvToAz { input, output } => env_to_az(input, output),
    };

    match result {
        Ok(_) => println!("Done."),
        Err(e) => println!("Error: {}", e),
    };
}

#[cfg(test)]
mod test {}
