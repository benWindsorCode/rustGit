use std::{env, fs};
use std::fs::{create_dir_all, metadata};
use std::path::{Path, PathBuf};
use crate::config::{Config, ConfigContents};
use crate::file_utils::{repo_dir, repo_file};

pub struct Repository {
    pub worktree: String,
    pub gitdir: String,
    pub conf: Config
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

        let config_path = Path::new(&config_file);
        if config_path.exists() {
            config.read().unwrap();
        } else if !force {
            panic!("Configuration file missing");
        }

        if !force {
            repository.version_check(&config.contents);
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
        repo.conf.write()?;

        Ok(repo)
    }

    pub fn find(path: String, required: bool) -> Result<Self, &'static str> {

        // TODO: this is a hack to handle '.' base case/edge case, implement relative paths here as the Path/PathBuf doesnt
        let mut path_to_search = path.clone();
        if path_to_search == "." {
            path_to_search = String::from(env::current_dir().unwrap().to_str().unwrap());
        }

        let mut path_obj = PathBuf::from(&path_to_search);
        println!("Searching for repo in: {:?}", path_obj);

        path_obj.push(".git");

        if path_obj.is_dir() {
            return Ok(Repository::new(path_to_search.clone(), false))
        };

        // push off the .git
        path_obj.pop();

        // now we are at the parent
        path_obj.pop();

        // TODO: this seemed to cause a stack overflow, in case no git dir at all, investigate
        if path_obj == Path::new("/") {
            if required {
                panic!("No git directory.")
            } else {
                return Err("Couldnt locate git directory in path")
            }
        }

        return Repository::find(String::from(path_obj.to_str().unwrap()), required);
    }

    fn version_check(&self, config_contents: &ConfigContents) {
        let version = config_contents.core.repository_format_version;

        if version != 0 {
            panic!("Unsupported repository_format_version {}", version);
        }
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
}