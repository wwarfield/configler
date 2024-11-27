use std::{collections::HashMap, str::FromStr};

use configler_core::{
    self,
    sources::{ConfigSource, YamlConfigSource},
    Config, ConfigBuilder, ConfigPropertyGroup, ConfigValueError, SourceName,
};

#[test]
fn verify_lazy_builder_and_config_visibility() {
    let builder_result = ConfigBuilder::new()
        .add_source(SourceName::Environment)
        .add_source(SourceName::YamlFile)
        .set_config_directory("./test_configs")
        .build();

    assert!(builder_result.is_ok());
}

#[test]
fn verify_builder_custom_source_visibility() {
    let yaml_source = YamlConfigSource::from_str(
        "
    database:
        user: baz
        password: foo
    ",
    )
    .unwrap();

    let builder_result = ConfigBuilder::new()
        .add_custom_source(Box::new(yaml_source))
        .build();

    assert!(builder_result.is_ok());

    let config = builder_result.unwrap();
    assert_eq!(
        config.get_value_string("database.user"),
        Some("baz".to_string())
    );
    assert_eq!(
        config.get_value_or_default("database.is_ssl", "true".to_string()),
        "true"
    );
}

#[test]
fn verify_custom_source_works_with_builder() {
    #[derive(Clone)]
    struct CustomSource {}

    impl ConfigSource for CustomSource {
        fn get_ordinal(&self) -> usize {
            10
        }

        fn get_value(&self, _property_name: &str) -> Option<String> {
            Some("example_value".to_string())
        }

        fn get_name(&self) -> &str {
            "custom test source"
        }

        fn from_file(
            _file_path: &str,
        ) -> Result<Self, configler_core::sources::config_source::FileError>
        where
            Self: Sized,
        {
            Ok(CustomSource {})
        }
    }

    let builder_result = ConfigBuilder::new()
        .add_custom_source(Box::new(CustomSource {}))
        .build();
    assert!(builder_result.is_ok());

    let config = builder_result.unwrap();
    assert_eq!(
        config.get_value_string("any.value"),
        Some("example_value".to_string())
    );
}

#[test]
fn verify_config_property_group_pattern() {
    let yaml_source = YamlConfigSource::from_str(
        "
    database:
        user: baz
        password: foo
    ",
    )
    .unwrap();

    let builder_result = ConfigBuilder::new()
        .add_custom_source(Box::new(yaml_source))
        .build();
    assert!(builder_result.is_ok());

    struct DbConfig<'a> {
        config: &'a Config,
    }
    impl<'a> DbConfig<'a> {
        fn get_username(&self) -> Result<String, ConfigValueError> {
            match self.config.get_value_string("database.user") {
                Some(value) => Ok(value),
                None => Err(ConfigValueError::NullError),
            }
        }

        fn get_password(&self) -> Result<String, ConfigValueError> {
            match self.config.get_value_string("database.password") {
                Some(value) => Ok(value),
                None => Err(ConfigValueError::NullError),
            }
        }
    }
    impl<'a> ConfigPropertyGroup<'a> for DbConfig<'a> {
        fn get_value_map(&self) -> Result<HashMap<String, Option<String>>, ConfigValueError> {
            // TODO it would be good to find a way to generalize this more
            let mut value_map: HashMap<String, Option<String>> = HashMap::new();
            match self.get_username() {
                Ok(value) => value_map.insert("DATABASE_USER".to_string(), Some(value)),
                Err(error) => return Err(error),
            };
            match self.get_password() {
                Ok(value) => value_map.insert("DATABASE_PASSWORD".to_string(), Some(value)),
                Err(error) => return Err(error),
            };
            Ok(value_map)
        }

        fn from_config(config: &'a Config) -> Self {
            DbConfig { config }
        }
    }

    let base_config = builder_result.unwrap();
    let db_config = DbConfig::from_config(&base_config);

    assert!(db_config.get_username().is_ok());
    assert_eq!(db_config.get_username().unwrap(), "baz");
    assert!(db_config.get_password().is_ok());
    assert_eq!(db_config.get_password().unwrap(), "foo");

    let value_map_result = db_config.get_value_map();
    assert!(value_map_result.is_ok());

    let value_map = value_map_result.unwrap();
    assert_eq!(
        value_map.get("DATABASE_USER"),
        Some(&Some("baz".to_string()))
    );
    assert_eq!(
        value_map.get("DATABASE_PASSWORD"),
        Some(&Some("foo".to_string()))
    );
}
