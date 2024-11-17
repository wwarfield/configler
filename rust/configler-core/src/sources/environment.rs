use super::{
    config_source::{convert_property_to_environment_name, FileError},
    ConfigSource,
};
use std::env;

#[derive(Clone)]
pub struct EnvironmentConfigSource {}

impl EnvironmentConfigSource {}

impl ConfigSource for EnvironmentConfigSource {
    fn get_ordinal(&self) -> usize {
        300
    }

    fn get_value(&self, property_name: &str) -> Option<String> {
        match env::var(convert_property_to_environment_name(property_name)) {
            Ok(value) => Some(value),
            Err(_) => None,
        }
    }

    fn get_name(&self) -> &str {
        std::any::type_name::<EnvironmentConfigSource>()
            .split("::")
            .last()
            .unwrap()
    }

    fn from_file(_file_path: &str) -> Result<Self, FileError> {
        Ok(EnvironmentConfigSource {})
    }
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn read_environment_variable() {
        env::set_var("TEST_ONE", "blah");

        let config_source = EnvironmentConfigSource {};
        let value = config_source.get_value("test.one");
        assert_ne!(value, None);
        assert_eq!(value.unwrap(), "blah");

        env::remove_var("TEST_ONE");
    }
}
