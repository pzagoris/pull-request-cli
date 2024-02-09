use serde::Deserialize;
use std::error::Error;
use std::fs::File;
use std::io::Read;

/// Github Configuration
#[derive(Debug, Deserialize, Clone)]
pub struct Config {
    /// base url of Github API.
    pub base_url: String,
    /// Owner of repository.
    pub repo_owner: String,
    /// Repositry name.
    pub repo_name: String,
    /// Github API token.
    pub github_token: String,
}

pub fn parse(file_path: &str) -> Result<Config, Box<dyn Error>> {
    let mut file = File::open(file_path)?;
    let mut buffer = String::new();
    file.read_to_string(&mut buffer)?;
    let deserialized: Config = serde_json::from_str(&buffer)?;
    Ok(deserialized)
}
