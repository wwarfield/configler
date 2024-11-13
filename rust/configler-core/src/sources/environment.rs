use super::ConfigSource;
use std::env;

#[derive(Clone)]
pub struct EnvironmentConfigSource {}

impl EnvironmentConfigSource {
    #![allow(dead_code)]
    fn convert_property_to_environment_name(&self, property_name: &str) -> String {
        // TODO add more conversion rules
        // https://smallrye.io/smallrye-config/Main/config/environment-variables/
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
            .split("::")
            .last()
            .unwrap()
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
    fn convert_property_to_environment_name(
        #[case] property_name: String,
        #[case] expected_env_name: String,
    ) {
        let config_source = EnvironmentConfigSource {};
        let env_name = config_source.convert_property_to_environment_name(&property_name);
        assert_eq!(expected_env_name, env_name);
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
}
