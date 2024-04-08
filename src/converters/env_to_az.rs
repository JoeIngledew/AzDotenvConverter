use std::{
    fs::{self, File},
    io::{self, BufRead},
    path::Path,
};

use crate::{az_types::AzOutputItem, errors::ConversionError};

fn read_lines<P>(filename: &P) -> io::Result<io::Lines<io::BufReader<File>>>
where
    P: AsRef<Path>,
{
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}

pub fn convert(input: &str, output: &str) -> Result<(), ConversionError> {
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
