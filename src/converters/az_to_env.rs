use std::{
    fs::{self, File},
    io::Read,
    path::Path,
};

use crate::{az_types::AzOutputItem, errors::ConversionError};

pub fn convert(input: &str, output: &str) -> Result<(), ConversionError> {
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
