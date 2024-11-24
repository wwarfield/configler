use std::{collections::HashMap, str::FromStr};

use yaml_rust2::{Yaml, YamlLoader};

use super::{config_source::FileError, ConfigSource};

#[derive(Clone)]
pub struct YamlConfigSource {
    // TODO not sure how I want to handle multi-docs since they could
    // clash
    yaml_docs: Vec<Yaml>
}

impl ConfigSource for YamlConfigSource {
    fn get_ordinal(&self) -> usize {
        265
    }

    fn get_value(&self, property_name: &str) -> Option<String> {
        // match env::var(convert_property_to_environment_name(property_name)) {
        //     Ok(value) => Some(value),
        //     Err(_) => None,
        // }
        None
    }

    fn get_name(&self) -> &str {
        // std::any::type_name::<EnvironmentConfigSource>()
        //     .split("::")
        //     .last()
        //     .unwrap()
        "hard coded"
    }

    fn from_file(_file_path: &str) -> Result<Self, FileError> {
        Ok(YamlConfigSource{
            yaml_docs: Vec::new()
        })
    }
}

impl FromStr for YamlConfigSource {
    // TODO clean up error
    type Err = String;

    fn from_str(yaml_str: &str) -> Result<Self, Self::Err> {
        let loader_result = YamlLoader::load_from_str(yaml_str);
        match loader_result {
            Ok(yaml_docs) => Ok(YamlConfigSource{yaml_docs: yaml_docs}),
            Err(scan_error) => {
                println!("{:?}", scan_error);
                // scan_error.info()
                Err("some value".to_string())
            },
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_yaml_string() {
        let yaml_str = "
        database:
            username: foo
            password: bar
        ";
        let config_result = YamlConfigSource::from_str(yaml_str);
        assert!(config_result.is_ok());

        let config_source = config_result.unwrap();
        let db_config = config_source.yaml_docs[0]["database"].clone();
        assert_eq!(db_config["username"].as_str(), Some("foo"));
        assert_eq!(db_config["password"].as_str(), Some("bar"));
    }

    #[test]
    fn parsing_invalid_yaml() {
        let invalid_str = "
            some value
            with other text which is obviously
            not yaml
        ";

        // even though this is not valid yaml, this library is particularly flexible
        // and still returns a valid result. This may be ok for now, but it could be worth
        // looking for other libraries that are more opinionated on yaml
        let config_result = YamlConfigSource::from_str(invalid_str);
        assert!(config_result.is_ok());

        let config = config_result.unwrap();
        assert_eq!(config.yaml_docs.len(), 1);
    }

    #[test]
    fn parse_multi_stanza_yaml_string() {
        let yaml_str = "
        database:
            username: foo
            password: bar
        endpoints:
            health: '/health'
            user: '/users'
        ";
        let config_result = YamlConfigSource::from_str(yaml_str);
        assert!(config_result.is_ok());

        let config_source = config_result.unwrap();
        assert_eq!(config_source.yaml_docs.len(), 1);

        assert!(!config_source.yaml_docs[0]["database"].is_null());
        assert!(!config_source.yaml_docs[0]["database"].is_null());
    }

    #[test]
    fn multi_yaml_doc_string() {
        let yaml_str = "
        database:
            user: foo
---
        endpoints:
            health: '/health'
        ";

        let config_result = YamlConfigSource::from_str(yaml_str);
        assert!(config_result.is_ok());

        let config = config_result.unwrap();
        //TODO I think multidoc should return an error
        assert_eq!(config.yaml_docs.len(), 2);
    }

    // TODO raise error for invalid yaml
    // TODO test parse from file
    // TODO test error when file does not exist
    // TODO get value from yaml
    // TODO get nested value from yaml
}