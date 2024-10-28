use std::env;

// sum 2 values and return string
pub fn sum_as_string(a: usize, b: usize) -> String {
    (a + b).to_string()
}

trait ConfigSource {
    fn get_ordinal(&self) -> usize;
    fn get_value(&self, property_name: String) -> Option<String>;
    fn get_name(&self) -> &str;
}

struct EnvironmentConfigSource {}

impl EnvironmentConfigSource {
    fn convert_property_to_environment_name(&self, property_name: String) -> String {
        str::replace(&property_name.to_uppercase(), ".", "_")
    }
}

impl ConfigSource for EnvironmentConfigSource {
    fn get_ordinal(&self) -> usize {
        300
    }
    
    fn get_value(&self, property_name: String) -> Option<String> {
        match env::var(self.convert_property_to_environment_name(property_name)) {
            Ok(value) => Some(value),
            Err(_) => None
        }
    }

    fn get_name(&self) -> &str {
        std::any::type_name::<EnvironmentConfigSource>()
    }
}

struct Config {
    sources: Vec<Box<dyn ConfigSource>>
}

impl Config {
    fn get_value(&self, property_name: &str) -> Option<String> {
        for config_source in self.sources.iter() {
            let value = config_source.get_value(property_name);
            if value.is_some() {
                return value
            }
        }
        None
    }

    fn get_value_or_default(&self, property_name: &str, default: String) -> String {
        match self.get_value(property_name) {
            Some(value) => value,
            None => default
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = sum_as_string(3, 2);
        assert_eq!(result, "5");
    }

    #[test]
    fn convert_property_to_environment_name() {
        let config_source = EnvironmentConfigSource{};
        let env_name = config_source.convert_property_to_environment_name("test.one".to_string());
        assert_eq!(env_name, "TEST_ONE");
    }

    #[test]
    fn read_environment_variable() {
        env::set_var("TEST_ONE", "blah");

        let config_source = EnvironmentConfigSource{};
        let value = config_source.get_value("test.one".to_string());
        assert_ne!(value, None);
        assert_eq!(value.unwrap(), "blah");

        env::remove_var("TEST_ONE");
    }
}
