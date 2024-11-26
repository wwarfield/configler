use std::str::FromStr;

use configler_core::{self, sources::{ConfigSource, YamlConfigSource}, ConfigBuilder, SourceName};

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
    let yaml_source = YamlConfigSource::from_str("
    database:
        user: baz
        password: foo
    ").unwrap();

    let builder_result = ConfigBuilder::new()
        .add_custom_source(Box::new(yaml_source))
        .build();

    assert!(builder_result.is_ok());

    let config = builder_result.unwrap();
    assert_eq!(config.get_value("database.user"), Some("baz".to_string()));
    assert_eq!(config.get_value_or_default("database.is_ssl", "true".to_string()), "true");
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
    
        fn from_file(_file_path: &str) -> Result<Self, configler_core::sources::config_source::FileError>
        where
            Self: Sized {
            Ok(CustomSource{})
        }
    }

    let builder_result = ConfigBuilder::new()
        .add_custom_source(Box::new(CustomSource{}))
        .build();
    assert!(builder_result.is_ok());

    let config = builder_result.unwrap();
    assert_eq!(config.get_value("any.value"), Some("example_value".to_string()));

}


// TODO verify pattern with sub config sources is possible