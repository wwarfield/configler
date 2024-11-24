pub mod sources;
use sources::{
    config_source::FileError, dot_env::DotEnvironmentConfigSource, ConfigSource,
    EnvironmentConfigSource,
};

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

#[derive(Debug)]
#[allow(dead_code)]
enum SourceName {
    Environment,
    DotEnvironmentFile,
}

struct ConfigBuilder {
    instantiated_sources: Vec<Box<dyn ConfigSource>>,
    lazy_sources: Vec<SourceName>,
    config_directory: Option<String>,
}

impl ConfigBuilder {
    #![allow(dead_code)]
    fn new() -> ConfigBuilder {
        ConfigBuilder {
            instantiated_sources: Vec::new(),
            lazy_sources: Vec::new(),
            config_directory: None,
        }
    }

    fn set_config_directory(&mut self, config_directory: &str) -> &mut Self {
        let mut directory = config_directory.to_owned();
        if !directory.ends_with("/") {
            directory += "/";
        }
        self.config_directory = Some(directory);
        self
    }

    fn add_source(&mut self, name: SourceName) -> &mut Self {
        self.lazy_sources.push(name);
        self
    }

    fn add_custom_source(&mut self, source: Box<dyn ConfigSource>) -> &mut Self {
        self.instantiated_sources.push(source);
        self
    }

    fn add_default_sources(&mut self) -> &mut ConfigBuilder {
        self.add_source(SourceName::Environment)
    }

    fn build(&self) -> Result<Config, FileError> {
        let mut final_sources = self.instantiated_sources.clone();

        if !self.lazy_sources.is_empty() {
            let env_source = EnvironmentConfigSource {};

            for source_name in self.lazy_sources.iter() {
                let config_source: Result<Box<dyn ConfigSource>, FileError> = match source_name {
                    SourceName::Environment => Ok(Box::new(EnvironmentConfigSource {})),
                    SourceName::DotEnvironmentFile => {
                        let file_path = env_source
                            .get_value("CONFIGLER_DOT_ENVIRONMENT_FILE")
                            .or(self.config_directory.clone())
                            .map_or(".env".to_string(), |path| path + ".env");

                        match DotEnvironmentConfigSource::from_file(&file_path) {
                            Ok(source) => Ok(Box::new(source)),
                            Err(error) => Err(error),
                        }
                    }
                };

                if let Ok(source) = config_source {
                    final_sources.push(source);
                } else {
                    return Err(config_source.err().unwrap());
                }
            }
        }

        // Sort sources by ascending ordinal value
        final_sources.sort_by_key(|s1| s1.as_ref().get_ordinal());

        Ok(Config {
            sources: final_sources,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use rstest::*;
    use std::{env, str::FromStr};

    #[test]
    fn it_works() {
        let result = sum_as_string(3, 2);
        assert_eq!(result, "5");
    }

    #[test]
    fn use_default_value() {
        // In this test the environment variable is not set
        let build_result = ConfigBuilder::new().add_default_sources().build();
        assert!(build_result.is_ok());

        let config = build_result.unwrap();

        let default_value = "default_value";
        let value = config.get_value_or_default("test.two", default_value.to_string());
        assert_eq!(value, default_value.to_string())
    }

    #[test]
    fn build_default_sources() {
        env::set_var("TEST_ONE", "blah");

        let build_result = ConfigBuilder::new().add_default_sources().build();
        assert!(build_result.is_ok());

        let config = build_result.unwrap();

        let value = config.get_value("test.one");
        assert_ne!(value, None);
        assert_eq!(value.unwrap(), "blah");
        env::remove_var("TEST_ONE");
    }

    #[test]
    fn custom_source_var() {
        let dot_env_str = "
        ONE_VAL=100
        TWO_VAL=300
        ";

        let build_result = ConfigBuilder::new()
            .add_custom_source(Box::new(
                DotEnvironmentConfigSource::from_str(dot_env_str).unwrap(),
            ))
            .build();
        assert!(build_result.is_ok());

        let config = build_result.unwrap();

        assert_eq!(config.get_value("one_val"), Some("100".to_string()));
        assert_eq!(config.get_value("two_val"), Some("300".to_string()));
    }

    #[rstest]
    #[case("test_configs")]
    #[case("test_configs/")]
    fn read_lazy_source_from_directory(#[case] directory: String) {
        let build_result = ConfigBuilder::new()
            .add_source(SourceName::DotEnvironmentFile)
            .set_config_directory(&directory)
            .build();
        assert!(build_result.is_ok());

        let config = build_result.unwrap();
        assert_eq!(config.get_value("KEY1"), Some("blah".to_string()));
    }

    #[test]
    fn overrides_respect_ordinal_values() {
        env::set_var("KEY1", "Overrided Value");

        let build_result = ConfigBuilder::new()
            .add_source(SourceName::DotEnvironmentFile)
            .add_source(SourceName::Environment)
            .set_config_directory("test_configs")
            .build();
        assert!(build_result.is_ok());

        let config = build_result.unwrap();

        assert_eq!(
            config.get_value("KEY1"),
            Some("Overrided Value".to_string())
        );

        env::remove_var("KEY1");
    }

    //TODO test environment variable overrides yaml source
}
