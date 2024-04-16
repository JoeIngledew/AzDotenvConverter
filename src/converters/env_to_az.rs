use crate::{az_types::AzOutputItem, errors::ConversionError};

pub fn convert(input_file_text: String) -> Result<String, ConversionError> {
    let lines = input_file_text.lines();
    let mut output_items: Vec<AzOutputItem> = Vec::new();
    let mut errors: Vec<usize> = Vec::new();
    let mut ix: usize = 0;
    for line in lines {
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
    Ok(new_file_text)
}

#[cfg(test)]
mod test {
    use std::env;

    use crate::{errors::ConversionError, filesystem::reader::get_file_text};

    use super::convert;

    #[test]
    fn test_convert() {
        let path = format!(
            "{}/src/test_files/valid-input.env",
            env::current_dir()
                .unwrap()
                .into_os_string()
                .into_string()
                .unwrap()
        );
        let input = get_file_text(&path).unwrap();
        let expected = "[{\"name\":\"KEY_ONE\",\"value\":\"abc\",\"slotSetting\":false},{\"name\":\"KEY_TWO\",\"value\":\"def\",\"slotSetting\":false}]".to_string();
        let result = convert(input).unwrap();
        assert_eq!(result, expected);
    }

    #[test]
    fn malformed_env_should_error() {
        let path = format!(
            "{}/src/test_files/invalid-input.env",
            env::current_dir()
                .unwrap()
                .into_os_string()
                .into_string()
                .unwrap()
        );
        let input = get_file_text(&path).unwrap();
        let result = convert(input).unwrap_err();
        assert!(matches!(result, ConversionError::Generic(_)))
    }
}
