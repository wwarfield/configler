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
        // let environment = EnvironmentConfigSource{}
        ConfigBuilder {
            sources: Vec::new(),
        }
    }

    fn add_source(&mut self, source: EnvironmentConfigSource) -> &mut ConfigBuilder {
        self.sources.push(Box::new(source));
        self
    }

    fn add_default_sources(&mut self) -> &mut ConfigBuilder {
        self.add_source(EnvironmentConfigSource {})
    }

    // TODO should be able to override config file path
    // TODO should be able to set config file path environment variable name

    // TODO this is where maybe it would be useful to separate out reading the config file out of source
    // instantiation. So that you can customize the source file location as part of the builder and then
    // when the configuration is built we maybe read the config files

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
