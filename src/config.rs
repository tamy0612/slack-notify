extern crate serde;
#[macro_use]
extern crate serde_derive;
extern crate serde_json;

use std::error::Error;
use std::fs::File;
use std::path::Path;
use serde_json::{Value, Error};


pub static DEFAULT_FILEPATH: String = String::from("~/.config/slack-notify/config.json");
static DEFAULT_USERNAME: String     = String::from("slack-notify");
static DEFAULT_ICON_EMOJI: String   = String::from(":ghost:");


#[derive(Serialize, Deserialize)]
pub struct Config {
    workspace: String,
    username: String,
    icon_emoji: String
}


impl Config {

    pub fn new() -> Option<Config> {
        let mut config = Config {
            workspace:  String::new(),
            username:   DEFAULT_USERNAME,
            icon_emoji: DEFAULT_ICON_EMOJI
        };
        Some(config)
    }

    pub fn load_file<P: AsRef<Path>>(path: P) -> Result<Config, Error> {
        File::open(path).and_then(|file| {
            serde_json::from_reader(file)
        })
    }

    pub fn save<P: AsRef<Path>>(&self, path: P) -> Result<()> {
        File::open(path).and_then(|file| {
            serde_json::ser::to_writer_pretty(path, &self)
        })
    }

    pub fn username(&self) -> &String
        &self.username
    }

    pub fn icon(&self) -> &String {
        &self.icon
    }

    pub fn workspace(&self, name: &str) -> &String {
        &self.workspace
    }

}
