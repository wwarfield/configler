use std::env;

use dyn_clone::DynClone;

// sum 2 values and return string
pub fn sum_as_string(a: usize, b: usize) -> String {
    (a + b).to_string()
}

trait ConfigSource: DynClone {
    fn get_ordinal(&self) -> usize;
    fn get_value(&self, property_name: &str) -> Option<String>;
    fn get_name(&self) -> &str;
}

dyn_clone::clone_trait_object!(ConfigSource);

#[derive(Clone)]
struct EnvironmentConfigSource {}

impl EnvironmentConfigSource {
    fn convert_property_to_environment_name(&self, property_name: &str) -> String {
        str::replace(&property_name.to_uppercase(), ".", "_")
    }
}

impl ConfigSource for EnvironmentConfigSource {
    fn get_ordinal(&self) -> usize {
        300
    }

    fn get_value(&self, property_name: &str) -> Option<String> {
        match env::var(self.convert_property_to_environment_name(property_name)) {
            Ok(value) => Some(value),
            Err(_) => None,
        }
    }

    fn get_name(&self) -> &str {
        std::any::type_name::<EnvironmentConfigSource>()
    }
}

struct Config {
    sources: Vec<Box<dyn ConfigSource>>,
}

impl Config {
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

    #[test]
    fn it_works() {
        let result = sum_as_string(3, 2);
        assert_eq!(result, "5");
    }

    #[test]
    fn convert_property_to_environment_name() {
        let config_source = EnvironmentConfigSource {};
        let env_name = config_source.convert_property_to_environment_name("test.one");
        assert_eq!(env_name, "TEST_ONE");
    }

    #[test]
    fn read_environment_variable() {
        env::set_var("TEST_ONE", "blah");

        let config_source = EnvironmentConfigSource {};
        let value = config_source.get_value("test.one");
        assert_ne!(value, None);
        assert_eq!(value.unwrap(), "blah");

        env::remove_var("TEST_ONE");
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
