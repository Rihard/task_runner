use serde_json::{Result, Value};
use serde::{Deserialize, Serialize};
use std::fs;

#[derive(Serialize, Deserialize)]
pub struct Config{
    apps: Vec<AppConfig>
}


#[derive(Serialize, Deserialize)]
pub struct AppConfig {
    name: String,
    trigger: Triggers,
    app_url: String,
    parameters_mode: ParametersMode
}


#[derive(Serialize, Deserialize)]
pub enum Triggers {
    StartWith ( String ),
    Contains ( String )
}


#[derive(Serialize, Deserialize)]
pub enum ParametersMode {
    PlainText
}

impl Config {
    pub fn load () -> Config {
        let file = fs::read_to_string("config.json").expect("Can't load config file!");
        serde_json::from_str(&file).expect("Can't parse config file!")
    }

}