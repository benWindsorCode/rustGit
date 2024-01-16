use std::fs;
use serde::{Deserialize, Serialize};

#[derive(Debug, PartialEq)]
pub struct Config {
    path: String,
    pub contents: ConfigContents
}

#[derive(Serialize, Deserialize, Debug, PartialEq)]
pub struct ConfigContents {
    pub core: CoreContents
}

#[derive(Serialize, Deserialize, Debug, PartialEq)]
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

    pub fn write(&self) -> Result<(), String> {
        let config_json = serde_json::to_string(&self.contents).or_else(|e| Err(e.to_string()))?;
        fs::write(&self.path, config_json).or_else(|e| Err(e.to_string()))
    }

    pub fn read(&mut self) -> Result<(), String> {
        let data = match fs::read_to_string(&self.path) {
            Ok(data) => data,
            Err(_) => return Err("Failed to read config".to_string())
        };

        serde_json::from_str(data.as_str()).and_then(|contents| {
            self.contents = contents;
            Ok(())
        }).or_else(|e| Err(e.to_string()))
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

#[cfg(test)]
mod test {
    use tempdir::TempDir;
    use crate::config::{Config, ConfigContents, CoreContents};

    #[test]
    fn config_write_read() {
        let tmp_dir = TempDir::new("dummy_repo").unwrap();
        let tmp_dir_string: String = tmp_dir.path().to_str().unwrap().into();
        let dummy_file_path = tmp_dir.path().join("some_file.txt");
        let dummy_file_path = dummy_file_path.as_path().to_str().unwrap().to_string();

        let contents = ConfigContents { core: CoreContents::new() };
        let config = Config { path: dummy_file_path.clone(), contents };

        let write_result = config.write();
        assert!(write_result.is_ok());

        let mut config_read = Config::new(dummy_file_path.clone());
        let read_result = config_read.read();
        assert!(read_result.is_ok());

        assert_eq!(config, config_read);
    }
}