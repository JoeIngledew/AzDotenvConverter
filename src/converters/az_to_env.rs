use crate::{az_types::AzOutputItem, errors::ConversionError};

pub fn convert(input_file_text: String) -> Result<String, ConversionError> {
    let items: Vec<AzOutputItem> = serde_json::from_str(&input_file_text)?;
    let output_items: Vec<String> = items
        .iter()
        .map(|x| format!("{}={}", x.name, x.value))
        .collect();

    let contents = output_items.join("\n");
    Ok(contents)
}

#[cfg(test)]
mod test {
    use crate::errors::ConversionError;

    use super::convert;

    #[test]
    fn test_convert() {
        let input = "[
            {
                \"name\": \"KEY_ONE\",
                \"value\": \"value one\",
                \"slotSetting\": false
            },
            {
                \"name\": \"KEY_TWO\",
                \"value\": \"value two\",
                \"slotSetting\": true
            }
        ]"
        .to_string();
        let expected = "KEY_ONE=value one
KEY_TWO=value two";

        let result = convert(input).unwrap();
        assert_eq!(result, expected);
    }

    #[test]
    fn invalid_json_error() {
        let input = "[
            {
                \"key\": \"KEY_ONE\",
                \"entry\": \"value one\",
                \"slotSetting\": false
            },
            {
                \"key\": \"KEY_TWO\",
                \"entry\": \"value two\",
                \"slotSetting\": true
            },
        ]"
        .to_string();

        let result = convert(input).unwrap_err();
        assert!(matches!(result, ConversionError::Json(_)))
    }

    #[test]
    fn incorrect_json_error() {
        let input = "not even JSON".to_string();

        let result = convert(input).unwrap_err();
        assert!(matches!(result, ConversionError::Json(_)))
    }
}
