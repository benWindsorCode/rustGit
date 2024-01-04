use std::fs::metadata;
use std::path::Path;
use crate::config::Config;

pub struct Repository {
    pub worktree: String,
    pub gitdir: String,
    // conf: Config
}

impl Repository {
    pub fn new(path: String) -> Self {
        let is_dir = metadata(path.clone()).unwrap().is_dir();

        if !is_dir {
            panic!("{} is not a directory", path.clone());
        }

        let path_obj = Path::new(&path);
        // let mut config = Config::new(path.clone());

        Repository {
            worktree: path.clone(),
            gitdir: String::from(path_obj.join(".git").to_str().unwrap())
        }
    }
}