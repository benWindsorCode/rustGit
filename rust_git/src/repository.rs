use std::fs;
use std::fs::{create_dir_all, metadata};
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
        let config = Config::new(config_file.clone());

        let config_path = Path::new(&config_file);
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

    pub fn create(path: String) -> Result<Self, &'static str> {
        let repo = Repository::new(path, true);

        let worktree = Path::new(&repo.worktree);
        let gitdir = Path::new(&repo.gitdir);

        if worktree.exists() {
            if !worktree.is_dir() {
                return Err("Path is not a directory");
            }

            if gitdir.exists() && !gitdir.read_dir().unwrap().next().is_none() {
                return Err("Path is not empty")
            }
        } else {
            create_dir_all(worktree).unwrap();
        }

        repo.create_dirs()?;
        repo.create_description()?;
        repo.create_head()?;
        repo.create_config()?;

        Ok(repo)
    }

    fn create_dirs(&self) -> Result<(), &'static str> {
        if repo_dir(&self, vec![String::from("branches")], true).is_none() {
            return Err("Couldnt create branches dir");
        }

        if repo_dir(&self, vec![String::from("objects")], true).is_none() {
            return Err("Couldnt create objects dir");
        }

        if repo_dir(&self, vec![String::from("refs"), String::from("tags")], true).is_none() {
            return Err("Couldnt create ./refs/tags dir");
        }

        if repo_dir(&self, vec![String::from("refs"), String::from("heads")], true).is_none() {
            return Err("Couldnt create ./refs/heads dir");
        }

        Ok(())
    }

    fn create_description(&self) -> Result<(), &'static str> {
        let file_name = repo_file(&self, vec![String::from("description")], false)?;

        fs::write(file_name, "Unnamed repository; edit this file 'description' to name the repository.\n").or_else(|e| {
            println!("{:?}", e);
            Err("Failed to create description")
        })
    }

    fn create_head(&self) -> Result<(), &'static str> {
        let file_name = repo_file(&self, vec![String::from("HEAD")], false)?;

        fs::write(file_name, "ref: refs/heads/master\n").or_else(|_| Err("Failed to write to HEAD file"))
    }

    fn create_config(&self) -> Result<(), &'static str> {
        let file_name = repo_file(&self, vec![String::from("config")], false)?;

        let config_json = serde_json::to_string(&self.conf).unwrap();
        fs::write(file_name, config_json).or_else(|_| Err("Failed to write to config file"))
    }
}