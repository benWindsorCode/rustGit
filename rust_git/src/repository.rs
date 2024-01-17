use std::{env, fs};
use std::fs::{create_dir_all, metadata};
use std::path::{Path, PathBuf};
use crate::config::{Config, ConfigContents};
use crate::file_utils::{repo_dir, repo_file};
use crate::index::Index;

#[derive(Debug)]
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

    pub fn create(path: String) -> Result<Self, String> {
        let repo = Repository::new(path, true);

        let worktree = Path::new(&repo.worktree);
        let gitdir = Path::new(&repo.gitdir);

        if worktree.exists() {
            if !worktree.is_dir() {
                return Err("Path is not a directory".to_string());
            }

            if gitdir.exists() && !gitdir.read_dir().unwrap().next().is_none() {
                return Err("Path is not empty".to_string())
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

    pub fn find(path: String, required: bool) -> Result<Self, String> {

        // TODO: this is a hack to handle '.' base case/edge case, implement relative paths here as the Path/PathBuf doesnt
        //         I suspect I need to use this https://doc.rust-lang.org/std/fs/fn.canonicalize.html
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
                return Err("Couldnt locate git directory in path".to_string())
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

    fn create_description(&self) -> Result<(), String> {
        let file_name = repo_file(&self, vec![String::from("description")], false)?;

        fs::write(file_name, "Unnamed repository; edit this file 'description' to name the repository.\n").or_else(|e| Err(e.to_string()))
    }

    fn create_head(&self) -> Result<(), String> {
        let file_name = repo_file(&self, vec![String::from("HEAD")], false)?;

        fs::write(file_name, "ref: refs/heads/master\n").or_else(|e| Err(e.to_string()))
    }

    /// Given a list of paths, remove their entries from the index if
    /// present, optionally delete the files if specified
    pub fn rm(&self, paths: Vec<String>, delete: bool, skip_missing: bool) {
        let index = Index::read(&self);

        todo!("Implement rm functionality")
    }

    pub fn add(&self, paths: Vec<String>, delete: bool, skip_missing: bool) {
        self.rm(paths.clone(), delete.clone(), skip_missing.clone());

        // TODO: Deal with the path cleaning

        let index = Index::read(&self);

        // TODO: use fs::metadata to get mtime etc. https://doc.rust-lang.org/std/fs/struct.Metadata.html
        // let entry = IndexEntry::new()

    }
}

#[cfg(test)]
mod tests {
    use tempdir::TempDir;
    use crate::repository::Repository;

    #[test]
    fn repo_create_new_and_find() {
        let tmp_dir = TempDir::new("dummy_repo").unwrap();
        let tmp_dir_string: String = tmp_dir.path().to_str().unwrap().into();

        // initialise an empty repo in the temp dir
        let repo = Repository::create(tmp_dir_string.clone());
        println!("Created test repo: {:?} in {:?}", repo, tmp_dir);
        assert!(repo.is_ok());

        // load that repo
        let repo_2 = Repository::new(tmp_dir_string.clone(), false);
        println!("Found repo: {:?}", repo_2);
        assert_eq!(repo_2.worktree, repo.unwrap().worktree);

        // search for that repo from the root dir
        let repo_3 = Repository::find(tmp_dir_string.clone(), true);
        assert!(repo_3.is_ok());
        assert_eq!(repo_3.unwrap().worktree, repo_2.worktree);

        // search for that repo from an inner dir
        let inner_dir = tmp_dir.path().join(".git/refs/heads");
        let inner_dir_string: String = inner_dir.as_path().to_str().unwrap().into();
        let repo_4 = Repository::find(inner_dir_string.clone(), true);
        assert!(repo_4.is_ok());
        assert_eq!(repo_4.unwrap().worktree, repo_2.worktree);

        tmp_dir.close().unwrap();
    }
}