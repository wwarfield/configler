use core::fmt;

use dyn_clone::DynClone;

use super::dot_env::DotEnvLineParseErrors;
use super::yaml::YamlParseError;

pub trait ConfigSource: DynClone {
    #![allow(dead_code)]
    fn get_ordinal(&self) -> usize;
    fn get_value(&self, property_name: &str) -> Option<String>;
    fn get_name(&self) -> &str;

    fn from_file(file_path: &str) -> Result<Self, FileError>
    where
        Self: Sized;
}

dyn_clone::clone_trait_object!(ConfigSource);

pub fn convert_property_to_environment_name(property_name: &str) -> String {
    // TODO add more conversion rules
    // https://smallrye.io/smallrye-config/Main/config/environment-variables/
    str::replace(&property_name.to_uppercase(), ".", "_")
}

#[derive(Debug)]
pub enum FileError {
    DotEnvLineParseErrors(DotEnvLineParseErrors),
    YamlParseError(YamlParseError),
    IoError(std::io::Error),
}

impl fmt::Display for FileError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            FileError::IoError(error) => write!(f, "{}", error),
            FileError::DotEnvLineParseErrors(error) => write!(f, "{}", error),
            FileError::YamlParseError(error) => write!(f, "{}", error),
        }
    }
}

#[cfg(test)]
mod tests {
    use rstest::*;

    use super::*;

    #[rstest]
    #[case("TEST.ONE", "TEST_ONE")]
    #[case("test.ONE", "TEST_ONE")]
    #[case("foo", "FOO")]
    // TODO adding more conversion rules
    // #[case("foo.\"bar\".baz", "FOO__BAR__BAZ")]
    // #[case("foo.bar-baz", "FOO_BAR_BAZ")]
    // #[case("foo.bar[0]", "FOO_BAR_0_")]
    // #[case("foo.bar[0].baz", "FOO_BAR_0__BAZ")]
    fn convert_property_to_environment_name_rules(
        #[case] property_name: String,
        #[case] expected_env_name: String,
    ) {
        let env_name = convert_property_to_environment_name(&property_name);
        assert_eq!(expected_env_name, env_name);
    }
}
