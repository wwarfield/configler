use super::ConfigSource;
use std::{collections::HashMap, env, error::Error, str::FromStr};

// https://www.dotenv.org/docs/security/env
#[derive(Clone)]
#[derive(Debug)]
pub struct DotEnvironmentConfigSource {
    // TODO I wonder if there is some value in separating the config source from actually storing
    // config values in a map, hmm
    values: HashMap<String, String>
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
        // TODO check what this looks like
        std::any::type_name::<DotEnvironmentConfigSource>()
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

        let result_key_value_pairs = dot_env_str
            .split("\n")
            // Filter out records that should be skipped
            .filter(|&record| {
                record.trim().chars().count() > 0
            })
            // Map record into key value pairs
            .map(|record| {
                let tokens = record.split('=').collect::<Vec<&str>>();
                if tokens.len() < 2 {
                    //TODO figure out how to add indices to error message
                    return Err("Record has invalid '=' operand")
                } else {
                    let key = tokens[0].trim();
                    let value = tokens[1].trim();
                    return Ok((key, value))
                }
            })
            .collect::<Vec<Result<(&str, &str), _>>>();

        let mut key_value_map: HashMap<String, String> = HashMap::new();
        for result_pair in result_key_value_pairs {
            if result_pair.is_err() {
                return Err(result_pair.err().unwrap().to_string())
            } else {
                let pair = result_pair.unwrap();
                key_value_map.insert(pair.0.to_owned(), pair.1.to_owned());
            }
        }
        Ok(DotEnvironmentConfigSource {
            values: key_value_map
        })
    }
    
    type Err = String;
}

#[cfg(test)]
mod tests {
    use rstest::*;

    use super::*;

    #[test]
    fn parse_simple_dot_env_string() {
        let dot_env_str = "
        FIRST=one
        FIRST_FOO=one
        ";

        let dot_env_source_result = DotEnvironmentConfigSource::from_str(dot_env_str);
        assert_eq!(dot_env_source_result.clone().err(), None);

        let dot_env_source = dot_env_source_result.unwrap();
        assert_eq!(dot_env_source.values.len(), 2);
    }

    #[test]
    fn parse_dot_env_string() {
        let dot_env_str = "
        FIRST=one
        FIRST_FOO=one
        # COMMENT
        export SECOND=three
        SPACE = space value
        QUOTED = \"quote value\"

        SECOND=second
        ";

        let dot_env_source_result = DotEnvironmentConfigSource::from_str(dot_env_str);
        assert_eq!(dot_env_source_result.clone().err(), None);

        let dot_env_source = dot_env_source_result.unwrap();
        assert_eq!(dot_env_source.values.len(), 2);
    }
}
