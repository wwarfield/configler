use std::env;

struct ConfigPOC {}

impl ConfigPOC {
    pub fn get_env_val(&self, name: String, default: String) -> String {
        env::var(name).unwrap_or(default)
    }
}

fn main() {
    let config = ConfigPOC {};
    println!(
        "Hello, world! TEST VALUE {}",
        config.get_env_val("TEST".to_string(), "default val".to_string())
    );
}
