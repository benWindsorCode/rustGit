use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct Config {
    path: String,
    pub contents: ConfigContents
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ConfigContents {
    pub core: CoreContents
}

#[derive(Serialize, Deserialize, Debug)]
pub struct CoreContents {
    pub repository_format_version: i8,
    pub filemode: bool,
    pub bare: bool
}

impl Config {
    pub fn new(path: String) -> Self {
        let config_contents = ConfigContents {
            core: CoreContents::new()
        };

        Config { path, contents: config_contents }
    }
}

impl CoreContents {
    pub fn new() -> Self {
        CoreContents {
            repository_format_version: 0,
            filemode: false,
            bare: false
        }
    }
}