use super::{config_source::convert_property_to_environment_name, ConfigSource};
use core::fmt;
use regex::Regex;
use std::{collections::HashMap, fs, str::FromStr};

// https://www.dotenv.org/docs/security/env
#[derive(Clone, Debug)]
#[allow(dead_code)]
pub struct DotEnvironmentConfigSource {
    // TODO I wonder if there is some value in separating the config source from actually storing
    // config values in a map, hmm --> Yes
    values: HashMap<String, String>,
}

impl ConfigSource for DotEnvironmentConfigSource {
    fn get_ordinal(&self) -> usize {
        295
    }

    fn get_value(&self, property_name: &str) -> Option<String> {
        let key = convert_property_to_environment_name(property_name);
        self.values.get(&key).map(|value| value.to_string())
    }

    fn get_name(&self) -> &str {
        std::any::type_name::<DotEnvironmentConfigSource>()
            .split("::")
            .last()
            .unwrap()
    }
}

#[allow(dead_code)]
impl DotEnvironmentConfigSource {
    fn from_file(file_path: &str) -> Result<Self, DotEnvFileError> {
        match fs::read_to_string(file_path) {
            Err(error) => Err(DotEnvFileError::IoError(error)),
            Ok(file_content) => match DotEnvironmentConfigSource::from_str(&file_content) {
                Err(parse_errors) => Err(DotEnvFileError::DotEnvLineParseErrors(parse_errors)),
                Ok(config_source) => Ok(config_source),
            },
        }
    }
}

impl FromStr for DotEnvironmentConfigSource {
    type Err = DotEnvLineParseErrors;

    fn from_str(dot_env_str: &str) -> Result<Self, Self::Err> {
        // Regular Expression splits text into records based on newlines while respecting multi-line quoted text
        let result_key_value_pairs = Regex::new(r#"(?:[^\n]+"[^"]*"\n)|(?:[^\n]*\n)|(?:[^\n]+$)"#)
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
                    Err((line_no, LineParseError::InvalidAssigment))
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
                        Err((line_no, LineParseError::KeyIsEmpty))
                    } else if value.is_empty() {
                        Err((line_no, LineParseError::ValueIsEmpty))
                    } else {
                        Ok((key, value))
                    }
                }
            })
            .collect::<Vec<Result<(String, &str), (usize, LineParseError)>>>();

        let mut parse_errors = DotEnvLineParseErrors {
            line_errors: Vec::new(),
        };
        let mut key_value_map: HashMap<String, String> = HashMap::new();
        for result_pair in result_key_value_pairs {
            if result_pair.is_err() {
                parse_errors.line_errors.push(result_pair.err().unwrap());
            } else {
                let pair = result_pair.unwrap();
                key_value_map.insert(pair.0.to_owned(), pair.1.to_owned());
            }
        }

        if parse_errors.line_errors.is_empty() {
            Ok(DotEnvironmentConfigSource {
                values: key_value_map,
            })
        } else {
            Err(parse_errors)
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct DotEnvLineParseErrors {
    line_errors: Vec<(usize, LineParseError)>,
}

#[derive(Debug, Clone, PartialEq)]
enum LineParseError {
    InvalidAssigment,
    KeyIsEmpty,
    ValueIsEmpty,
}

impl fmt::Display for DotEnvLineParseErrors {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        writeln!(
            f,
            "Found {} errors while parsing Dot Environment File",
            self.line_errors.len()
        )?;
        for (line_number, parse_error) in self.line_errors.iter() {
            let error_description = match parse_error {
                LineParseError::InvalidAssigment => "Record has invalid '=' operand",
                LineParseError::KeyIsEmpty => "key is empty",
                LineParseError::ValueIsEmpty => "value is empty",
            };
            writeln!(f, "Line {}: {}", line_number, error_description)?;
        }
        Ok(())
    }
}

#[derive(Debug)]
enum DotEnvFileError {
    DotEnvLineParseErrors(DotEnvLineParseErrors),
    IoError(std::io::Error),
}

impl fmt::Display for DotEnvFileError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            DotEnvFileError::IoError(error) => write!(f, "{}", error),
            DotEnvFileError::DotEnvLineParseErrors(error) => write!(f, "{}", error),
        }
    }
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
        let parse_errors = dot_env_source_result.err();

        let expected_parse_errors = DotEnvLineParseErrors {
            line_errors: vec![(2, LineParseError::ValueIsEmpty)],
        };
        assert_eq!(parse_errors, Some(expected_parse_errors));
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

        let parse_errors = dot_env_source_result.err();

        let expected_parse_errors = DotEnvLineParseErrors {
            line_errors: vec![
                (2, LineParseError::ValueIsEmpty),
                (4, LineParseError::KeyIsEmpty),
            ],
        };
        assert_eq!(parse_errors, Some(expected_parse_errors));
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
        assert_eq!(
            get_config_value(&dot_env_source, "SECOND"),
            Some("blah".to_string())
        )
    }

    #[test]
    fn parse_config_without_newlines() {
        let dot_env_str = "FIRST=one";

        let dot_env_source_result = DotEnvironmentConfigSource::from_str(dot_env_str);
        assert_eq!(dot_env_source_result.clone().err(), None);

        let dot_env_source = dot_env_source_result.unwrap();
        assert_eq!(dot_env_source.values.len(), 1);

        assert_eq!(
            get_config_value(&dot_env_source, "FIRST"),
            Some("one".to_string())
        );
    }

    #[test]
    fn parse_from_file() {
        let dot_config_result = DotEnvironmentConfigSource::from_file("./test.env");
        assert!(dot_config_result.is_ok());

        let dot_config_source = dot_config_result.unwrap();

        assert_eq!(dot_config_source.values.len(), 1);
        assert_eq!(
            get_config_value(&dot_config_source, "KEY1"),
            Some("blah".to_string())
        );
    }

    #[test]
    fn errors_out_when_file_does_not_exist() {
        let dot_config_result = DotEnvironmentConfigSource::from_file("./fake-file.env");
        assert!(dot_config_result.is_err());
        let config_error = dot_config_result.err().unwrap();
        assert!(matches!(config_error, DotEnvFileError::IoError(_)));
    }

    #[test]
    fn config_source_name() {
        let dot_env_str = "FIRST=one";

        let dot_env_source_result = DotEnvironmentConfigSource::from_str(dot_env_str);
        assert_eq!(dot_env_source_result.clone().err(), None);

        let dot_env_source = dot_env_source_result.unwrap();
        assert_eq!(dot_env_source.get_name(), "DotEnvironmentConfigSource");
    }

    #[test]
    fn read_dot_env_variable() {
        let dot_env_str = "
        FIRST=first_value
        SECOND=other value
        ";
        let dot_env_source = DotEnvironmentConfigSource::from_str(dot_env_str).unwrap();

        println!("values: {:?}", dot_env_source.values);
        let value = dot_env_source.get_value("first");
        assert_eq!(value, Some("first_value".to_string()));
    }
}
