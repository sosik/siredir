use std::borrow::Cow;

use serde_derive;
use serde_yaml;

// Thread safe shareable representation of current configuration
// Representation of config file
#[derive(serde_derive::Deserialize, Debug)]
pub struct Config {
    bind_to: Vec<String>,
    redirects: Vec<ConfigRedirectRecord>,
}

#[derive(serde_derive::Deserialize, Debug, Clone)]
pub struct ConfigRedirectRecord {
    pub re: String,
    pub rewrite_rule: String,
    pub status_code: u16,
}

impl Config {
    pub fn new() -> Self {
        Self::default()
    }

    // Load configuration form yaml file
    pub fn load<P: AsRef<std::path::Path>>(path: P) -> Result<Self, Box<std::error::Error>> {
        let f = std::fs::File::open(path)?;

        Ok(serde_yaml::from_reader(f)?)
    }

    pub fn get_bind_to(&self) -> Cow<Vec<String>> {
        Cow::Borrowed(&self.bind_to)
    }

    pub fn get_redirects(&self) -> Cow<Vec<ConfigRedirectRecord>> {
        Cow::Borrowed(&self.redirects)
    }
}

impl Default for Config {
    fn default() -> Self {
        Self {
            bind_to: vec![String::from("0.0.0.0:8080")],
            redirects: vec![],
        }
    }
}
