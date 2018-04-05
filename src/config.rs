use std::io::{Read, Error as IOError};
use std::fs::File;
use std::collections::HashMap;

extern crate url;
extern crate toml;

pub struct Config {
    username: String,
    icon: String,
    workspace: HashMap<String, url::Url>
}

pub enum LoadError {
    Io(IOError),
    Parse(toml::de::Error),
    Undefined(String)
}

impl Config {
    pub fn load(path: &str) -> Result<Config, LoadError> {
        Config::load_toml(&path).and_then(|toml| {
            let _username = match toml.get("username") {
                Some(value) => value.as_str().unwrap(),
                None => "slack_bot"
            };
            let _icon = match toml.get("icon") {
                Some(value) => value.as_str().unwrap(),
                None => ""
            };
            let mut config = Config {
                username: _username.to_string(),
                icon: _icon.to_string(),
                workspace: HashMap::new()
            };
            let workspace = toml.get("workspace")
                                .and_then(|value| {value.as_array()});
            for value in workspace {
                let name = value.get("name").and_then(|v| {v.as_str()}).unwrap().to_string();
                let url = url::Url::from_str(value.get("url").and_then(|v| {v.as_str()}).unwrap());
                config.workspace.insert(name, url);
            }
            Ok(config)
        })
    }

    pub fn save(_path: &str) -> bool {
        false
    }

    pub fn username(&self) -> &String {
        &self.username
    }

    pub fn icon(&self) -> &String {
        &self.icon
    }

    pub fn workspace(&self, name: &str) -> Option<&url::Url> {
        self.workspace.get(&name.to_string())
    }

    fn load_toml(path: &str) -> Result<toml::Value, LoadError> {
        match Config::load_file(&path) {
            Ok(toml_string) => Config::parse_toml(&toml_string),
            Err(e) => Err(LoadError::Io(e))
        }
    }

    fn load_file(path: &str) -> Result<String, IOError> {
        File::open(&path).and_then(|mut file| {
            let mut toml_string = String::new();
            file.read_to_string(&mut toml_string).and_then(|_| {
                Ok(toml_string)
            })
        })
    }

    fn parse_toml(toml_string: &String) -> Result<toml::Value, LoadError> {
        match toml_string.parse::<toml::Value>() {
            Err(e) => Err(LoadError::Parse(e)),
            Ok(toml) => Ok(toml)
        }
    }

}


use std::string::ToString;

impl ToString for Config {
    fn to_string(&self) -> String {
        let mut ret = String::new();
        ret
    }
}
