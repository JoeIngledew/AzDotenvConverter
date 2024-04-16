pub(crate) mod az_to_env;
pub(crate) mod env_to_az;

#[cfg(test)]
mod test {
    use super::{az_to_env, env_to_az};

    #[test]
    fn identity_az_to_env_to_az() {
        let initial = "[{\"name\":\"KEY_ONE\",\"value\":\"abc\",\"slotSetting\":false},{\"name\":\"KEY_TWO\",\"value\":\"def\",\"slotSetting\":false}]".to_string();
        let to_env = az_to_env::convert(initial.clone()).unwrap();
        let to_az = env_to_az::convert(to_env).unwrap();
        assert_eq!(initial, to_az);
    }

    #[test]
    fn identity_env_to_az_to_env() {
        let initial = "KEY_ONE=value one
KEY_TWO=value two"
            .to_string();
        let to_az = env_to_az::convert(initial.clone()).unwrap();
        let to_env = az_to_env::convert(to_az).unwrap();
        assert_eq!(initial, to_env)
    }
}
