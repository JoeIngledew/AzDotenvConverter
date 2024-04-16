use std::{fs::write, path::Path};

use crate::errors::ConversionError;

pub fn write_file(output_path: &str, contents: String) -> Result<(), ConversionError> {
    let output_path = Path::new(output_path);

    write(output_path, contents).map_err(|e| ConversionError::WriteError(e.to_string()))?;
    Ok(())
}
