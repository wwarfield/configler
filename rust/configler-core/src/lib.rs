pub mod sources;
use sources::{ConfigSource, EnvironmentConfigSource};

// sum 2 values and return string
pub fn sum_as_string(a: usize, b: usize) -> String {
    (a + b).to_string()
}

struct Config {
    sources: Vec<Box<dyn ConfigSource>>,
}

impl Config {
    #![allow(dead_code)]
    fn get_value(&self, property_name: &str) -> Option<String> {
        for config_source in self.sources.iter() {
            let value = config_source.get_value(property_name);
            if value.is_some() {
                return value;
            }
        }
        None
    }

    fn get_value_or_default(&self, property_name: &str, default: String) -> String {
        match self.get_value(property_name) {
            Some(value) => value,
            None => default,
        }
    }
}

struct ConfigBuilder {
    sources: Vec<Box<dyn ConfigSource>>,
}

impl ConfigBuilder {
    #![allow(dead_code)]
    fn new() -> ConfigBuilder {
        ConfigBuilder {
            sources: Vec::new(),
        }
    }

    fn add_default_sources(&mut self) -> &mut ConfigBuilder {
        self.sources.push(Box::new(EnvironmentConfigSource {}));
        self
    }

    fn build(&self) -> Config {
        Config {
            sources: self.sources.clone(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::env;

    #[test]
    fn it_works() {
        let result = sum_as_string(3, 2);
        assert_eq!(result, "5");
    }

    #[test]
    fn use_default_value() {
        // In this test the environment variable is not set
        let config = ConfigBuilder::new().add_default_sources().build();

        let default_value = "default_value";
        let value = config.get_value_or_default("test.two", default_value.to_string());
        assert_eq!(value, default_value.to_string())
    }

    #[test]
    fn build_default_sources() {
        env::set_var("TEST_ONE", "blah");

        let config = ConfigBuilder::new().add_default_sources().build();

        let value = config.get_value("test.one");
        assert_ne!(value, None);
        assert_eq!(value.unwrap(), "blah");
        env::remove_var("TEST_ONE");
    }
}
