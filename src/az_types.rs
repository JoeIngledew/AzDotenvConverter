use serde::{Deserialize, Serialize};

use crate::errors::ConversionError;

#[derive(Serialize, Deserialize, PartialEq, Eq, PartialOrd, Ord, Debug)]
#[serde(rename_all = "camelCase")]
pub struct AzOutputItem {
    pub name: String,
    pub value: String,
    slot_setting: bool,
}

impl AzOutputItem {
    pub fn new(name: &str, value: &str) -> Self {
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

#[cfg(test)]
mod test {
    use fake::{Fake, StringFaker};

    use crate::errors::ConversionError;
    use crate::az_types::AzOutputItem;

    #[derive(Debug, Clone)]
    struct SafeName(pub String);

    #[derive(Debug, Clone)]
    struct SafeValue(pub String);

    const VALID_CHARS_VALUE: &str =
        "0123456789ABCDEFGHIJKLMNOPQRSTUVWXYZabvdefghijklmnopqrstuvwxyz[]{}_+/\\<>,.`~!@£$%^&*()";
    const VALID_CHARS_NAME: &str = "ABCDEFGHIJKLMNOPQRSTUVWXYZ_0123456789";

    impl quickcheck::Arbitrary for SafeName {
        fn arbitrary(_g: &mut quickcheck::Gen) -> Self {
            let s = StringFaker::with(Vec::from(VALID_CHARS_NAME), 1..30);
            SafeName(s.fake())
        }
    }

    impl quickcheck::Arbitrary for SafeValue {
        fn arbitrary(_g: &mut quickcheck::Gen) -> Self {
            let s = StringFaker::with(Vec::from(VALID_CHARS_VALUE), 1..30);
            SafeValue(s.fake())
        }
    }

    #[quickcheck_macros::quickcheck]
    fn try_from_valid_input_should_return_correct_output(s1: SafeName, s2: SafeValue) {
        let str = format!("{}={}", &s1.0, &s2.0);
        let result = AzOutputItem::try_from(str).unwrap();
        let expected = AzOutputItem::new(&s1.0, &s2.0);
        assert_eq!(result, expected);
    }

    #[quickcheck_macros::quickcheck]
    fn try_from_invalid_input_should_return_correct_output(s1: SafeName, s2: SafeValue) {
        let str = format!("{}{}", &s1.0, &s2.0);
        let result = AzOutputItem::try_from(str);
        assert!(result.is_err_and(|e| matches!(e, ConversionError::Generic(_))))
    }
}
