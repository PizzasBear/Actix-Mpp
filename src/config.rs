use serde::Deserialize;
use std::fs::File;
use std::io;

#[derive(Debug, Deserialize)]
pub struct Config {
    pub address: String,
    pub pg_config: String,
}

impl Config {
    pub fn read(path: &str) -> std::io::Result<Self> {
        let file = File::open(path)?;

        match ron::de::from_reader::<File, Self>(file) {
            Ok(s) => Ok(s),
            Err(e) => Err(io::Error::new(io::ErrorKind::InvalidData, e)),
        }
    }
}
