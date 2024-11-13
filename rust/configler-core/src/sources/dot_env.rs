use super::ConfigSource;
use regex::Regex;
use std::{collections::HashMap, env, str::FromStr};

// https://www.dotenv.org/docs/security/env
#[derive(Clone, Debug)]
#[allow(dead_code)]
pub struct DotEnvironmentConfigSource {
    // TODO I wonder if there is some value in separating the config source from actually storing
    // config values in a map, hmm
    values: HashMap<String, String>,
}

impl ConfigSource for DotEnvironmentConfigSource {
    fn get_ordinal(&self) -> usize {
        295
    }

    fn get_value(&self, property_name: &str) -> Option<String> {
        match env::var(self.convert_property_to_environment_name(property_name)) {
            Ok(value) => Some(value),
            Err(_) => None,
        }
    }

    fn get_name(&self) -> &str {
        std::any::type_name::<DotEnvironmentConfigSource>()
            .split("::")
            .last()
            .unwrap()
    }
}

impl DotEnvironmentConfigSource {
    #![allow(dead_code)]
    fn convert_property_to_environment_name(&self, property_name: &str) -> String {
        // TODO add more conversion rules
        // https://smallrye.io/smallrye-config/Main/config/environment-variables/
        str::replace(&property_name.to_uppercase(), ".", "_")
    }
}

impl FromStr for DotEnvironmentConfigSource {
    fn from_str(dot_env_str: &str) -> Result<Self, Self::Err> {
        // TODO break up pipeline into parsing functions?
        // TODO pre-compile expression

        // Regular Expression splits text into records based on newlines while respecting multi-line quoted text
        let result_key_value_pairs = Regex::new(r#"(?:[^\n]+"[^"]*"\n)|(?:[^\n]*\n)"#)
            .unwrap()
            .find_iter(dot_env_str)
            .map(|m| m.as_str())
            // enumerate records into line_no & record
            .enumerate()
            // Filter out records that should be skipped
            .filter(|(_i, record)| {
                let trimmed_record = record.trim();
                let is_empty = trimmed_record.chars().count() == 0;
                let is_comment = trimmed_record.starts_with("#");
                !is_empty && !is_comment
            })
            // Map record into key value pairs
            .map(|(line_no, record)| {
                let tokens = record.split('=').collect::<Vec<&str>>();
                if tokens.len() < 2 {
                    Err(format!(
                        "At Line {}, Record has invalid '=' operand",
                        line_no
                    ))
                } else {
                    let key = tokens[0]
                        .trim()
                        .trim_start_matches("export ")
                        .trim()
                        .to_uppercase();

                    let temp_value = tokens[1].trim();
                    let value: &str = if temp_value.starts_with("\"") && temp_value.ends_with("\"")
                    {
                        temp_value.trim_start_matches("\"").trim_end_matches("\"")
                    } else {
                        temp_value
                    };

                    if key.is_empty() {
                        return Err(format!("At Line {}, Key is empty", line_no));
                    }
                    if value.is_empty() {
                        return Err(format!("At Line {}, value is empty", line_no));
                    }

                    Ok((key, value))
                }
            })
            .collect::<Vec<Result<(String, &str), _>>>();

        let mut error_message = "".to_owned();
        let mut key_value_map: HashMap<String, String> = HashMap::new();
        for result_pair in result_key_value_pairs {
            if result_pair.is_err() {
                if !error_message.is_empty() {
                    error_message.push('\n');
                }
                error_message.push_str(&result_pair.err().unwrap().to_string());
            } else {
                let pair = result_pair.unwrap();
                key_value_map.insert(pair.0.to_owned(), pair.1.to_owned());
            }
        }

        if !error_message.is_empty() {
            Err(error_message)
        } else {
            Ok(DotEnvironmentConfigSource {
                values: key_value_map,
            })
        }
    }

    // TODO should there be a custom Error Type?
    type Err = String;
}

#[cfg(test)]
mod tests {
    use super::*;

    fn get_config_value(config: &DotEnvironmentConfigSource, key: &str) -> Option<String> {
        config.values.get(key).map(|s| s.to_string())
    }

    #[test]
    fn parse_simple_dot_env_string() {
        let dot_env_str = "
        FIRST=one
        FIRST_FOO=one foo
        ";

        // Verify no parsing errors
        let dot_env_source_result = DotEnvironmentConfigSource::from_str(dot_env_str);
        assert_eq!(dot_env_source_result.clone().err(), None);

        // verify the correct number of items were added
        let dot_env_source = dot_env_source_result.unwrap();
        assert_eq!(dot_env_source.values.len(), 2);

        // verify key values
        assert_eq!(
            get_config_value(&dot_env_source, "FIRST"),
            Some("one".to_string())
        );
        assert_eq!(
            get_config_value(&dot_env_source, "FIRST_FOO"),
            Some("one foo".to_string())
        );
    }

    #[test]
    fn parse_dot_env_string() {
        let dot_env_str = "
        FIRST=one
        # COMMENT
        export SECOND=second value
        SPACE = space value
        QUOTED = \"quote value\"

        THIRD=third
        lower=fourth value
        UPPER=UPPER VALUE
        ";

        let dot_env_source_result = DotEnvironmentConfigSource::from_str(dot_env_str);
        assert_eq!(dot_env_source_result.clone().err(), None);

        let dot_env_source = dot_env_source_result.unwrap();
        assert_eq!(dot_env_source.values.len(), 7);

        // verify key values
        assert_eq!(
            get_config_value(&dot_env_source, "FIRST"),
            Some("one".to_string())
        );
        assert_eq!(
            get_config_value(&dot_env_source, "SECOND"),
            Some("second value".to_string())
        );
        assert_eq!(
            get_config_value(&dot_env_source, "SPACE"),
            Some("space value".to_string())
        );
        assert_eq!(
            get_config_value(&dot_env_source, "QUOTED"),
            Some("quote value".to_string())
        );
        assert_eq!(
            get_config_value(&dot_env_source, "THIRD"),
            Some("third".to_string())
        );
        assert_eq!(
            get_config_value(&dot_env_source, "LOWER"),
            Some("fourth value".to_string())
        );
        assert_eq!(
            get_config_value(&dot_env_source, "UPPER"),
            Some("UPPER VALUE".to_string())
        );
    }

    #[test]
    fn parse_invalid_dot_env_error() {
        let dot_env_str = "
        FIRST=one
        BAD_FOO=
        SECOND=la
        ";

        // Verify parsing error
        let dot_env_source_result = DotEnvironmentConfigSource::from_str(dot_env_str);
        assert_eq!(
            dot_env_source_result.clone().err(),
            Some("At Line 2, value is empty".to_string())
        );
    }

    #[test]
    fn parse_multiple_dot_env_errors() {
        let dot_env_str = "
        FIRST=one
        BAD_FOO=
        SECOND=la
        =blah
        ";

        // Verify parsing error
        let dot_env_source_result = DotEnvironmentConfigSource::from_str(dot_env_str);
        assert_eq!(
            dot_env_source_result.clone().err(),
            Some("At Line 2, value is empty\nAt Line 4, Key is empty".to_string())
        );
    }

    #[test]
    fn parse_multiline_quoted_value() {
        let dot_env_str = "
        FIRST=\"some value
        extends onto multiple lines
        then ends\"
        SECOND=blah
        ";

        // Verify parsing error
        let dot_env_source_result = DotEnvironmentConfigSource::from_str(dot_env_str);
        assert_eq!(dot_env_source_result.clone().err(), None);

        let dot_env_source = dot_env_source_result.unwrap();
        assert_eq!(dot_env_source.values.len(), 2);
        assert_eq!(
            get_config_value(&dot_env_source, "FIRST"),
            Some("some value\n        extends onto multiple lines\n        then ends".to_string())
        );
        // TODO may want a helper function to make these tests more readable
        assert_eq!(
            get_config_value(&dot_env_source, "SECOND"),
            Some("blah".to_string())
        )
    }

    #[test]
    fn config_source_name() {
        let dot_env_str = "FIRST=one";

        // verify no parsing error
        let dot_env_source_result = DotEnvironmentConfigSource::from_str(dot_env_str);
        assert_eq!(dot_env_source_result.clone().err(), None);

        // verify the correct number of items were added
        let dot_env_source = dot_env_source_result.unwrap();
        assert_eq!(dot_env_source.get_name(), "DotEnvironmentConfigSource");
    }
}
