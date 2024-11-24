use std::{
    collections::HashMap,
    fs::{self, File},
    str::FromStr,
};

use yaml_rust2::{Yaml, YamlLoader};

use super::{config_source::FileError, ConfigSource};

#[derive(Clone)]
pub struct YamlConfigSource {
    yaml_doc: Yaml,
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

    fn from_file(file_path: &str) -> Result<Self, FileError> {
        match fs::read_to_string(file_path) {
            Err(error) => Err(FileError::IoError(error)),
            Ok(file_content) => match YamlConfigSource::from_str(&file_content) {
                //TODO this should be a parse error not a FileError
                Err(parse_error) => Err(parse_error),
                Ok(config_source) => Ok(config_source),
            },
        }
    }
}

impl FromStr for YamlConfigSource {
    type Err = FileError;

    fn from_str(yaml_str: &str) -> Result<Self, Self::Err> {
        let loader_result = YamlLoader::load_from_str(yaml_str);
        match loader_result {
            Ok(yaml_docs) => {
                if yaml_docs.len() > 1 {
                    // Yaml Multi-docs allow there to be conflicting keys
                    // in the Yaml file which would make resolving key names
                    // ambiguous. To avoid that we will not support multi-doc
                    return Err(FileError::YamlUnsupportedMultiDoc);
                }
                Ok(YamlConfigSource {
                    yaml_doc: yaml_docs[0].clone(),
                })
            }
            Err(scan_error) => Err(FileError::YamlScanError(scan_error)),
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
        let db_config = config_source.yaml_doc["database"].clone();
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

        assert!(!config_source.yaml_doc["database"].is_null());
        assert!(!config_source.yaml_doc["database"].is_null());
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
        assert!(config_result.is_err());
    }

    #[test]
    fn invalid_colon_position() {
        let yaml_str = "
        database:
            user: foo
            missing colon
        ";

        let config_result = YamlConfigSource::from_str(yaml_str);
        assert!(config_result.is_err());
    }

    #[test]
    fn parse_yaml_file() {
        let config_result = YamlConfigSource::from_file("./test_configs/test.yaml");
        assert!(config_result.is_ok());

        let config = config_result.unwrap();
        assert!(!config.yaml_doc["database"].is_null());
        assert!(!config.yaml_doc["app"].is_null());
    }

    #[test]
    fn missing_file_error() {
        let config_result = YamlConfigSource::from_file("./non-existent-file.yaml");
        assert!(config_result.is_err());
    }

    // TODO get value from yaml
    // TODO get nested value from yaml
}
