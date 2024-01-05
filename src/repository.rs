use std::fs::metadata;
use std::path::Path;
use crate::config::Config;
use crate::utils::{repo_dir, repo_file};

pub struct Repository {
    pub worktree: String,
    pub gitdir: String,
    conf: Config
}

impl Repository {
    pub fn new(path: String, force: bool) -> Self {
        let is_dir = metadata(path.clone()).unwrap().is_dir();

        if !is_dir {
            panic!("{} is not a directory", path.clone());
        }

        let path_obj = Path::new(&path);

        let mut repository = Repository {
            worktree: path.clone(),
            gitdir: String::from(path_obj.join(".git").to_str().unwrap()),
            // for now make a dummy config which we then populate (but the populate functions need a repo)
            conf: Config::new(String::from(""))
        };

        let config_file = repo_file(&repository, vec![String::from("config")], false).unwrap();
        let mut config = Config::new(config_file.clone());

        let config_path = Path::new(&config_file.clone());
        if config_path.exists() {
            //read config
        } else if !force {
            panic!("Configuration file missing")
        }

        if !force {
            let version = config.contents.core.repository_format_version;

            if version != 0 {
                panic!("Unsupported repository_format_version {}", version);
            }
        }

        repository.conf = config;
        repository
    }
}