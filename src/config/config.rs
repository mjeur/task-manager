use std::fs::File;
use std::io::Read;
use serde_derive::{Deserialize, Serialize};
use serde_json;

#[derive(Deserialize, Serialize)]
pub struct Config {
    pub server: Server,
    pub database: Database,
}

#[derive(Deserialize, Serialize)]
pub struct Server {
    pub host: String,
    pub port: u16,
}

#[derive(Deserialize, Serialize)]
pub struct Database {
    pub url: String,
    pub username: String,
    pub password: String,
}

impl Config {
    pub fn new(file_path: &str) -> Result<Self, serde_json::Error> {
        let mut file = match File::open(file_path) {
            Ok(file) => file,
            Err(err) => return Err(serde_json::Error::io(err)),
        };
        let mut contents = String::new();
        match file.read_to_string(&mut contents) {
            Ok(_) => (),
            Err(err) => return Err(serde_json::Error::io(err)),
        }
        serde_json::from_str(&contents)
    }
}