use std::fs;
use std::path::Component::ParentDir;
use std::path::{Path, PathBuf};
use std::time::SystemTime;
use serde::{Deserialize, Serialize};
use crate::file_utils::repo_file;
use crate::repository::Repository;

#[derive(Serialize, Deserialize, Debug)]
pub struct Index {
    version: i32,
    entries: Vec<IndexEntry>
}

#[derive(Serialize, Deserialize, Debug)]
pub struct IndexEntry {
    // TODO: is this the right way to store time?
    time: SystemTime,
    mtime: SystemTime,
    dev: String,
    // TODO: could this be u32?
    ino: i32,
    model_type: ModelType,
    model_perms: i32,
    uid: i32,
    gid: i32,
    fsize: i32,
    sha: String,
    flag_assume_valid: bool,
    flag_stage: bool,
    name: String
}

#[derive(Serialize, Deserialize, Debug)]
enum ModelType {
    Regular,
    Symlink,
    Gitlink
}

impl Index {
    pub fn new() -> Self {
        Index { version: 2, entries: Vec::new() }
    }

    pub fn read(repo: &Repository) -> Result<Self, String> {
        let index_path = Index::path(&repo);

        fs::read(index_path).as_ref().and_then(|data| Ok(serde_json::from_slice(data).unwrap()))
            .map_err(|e| e.to_string())
    }

    pub fn write(&self, repo: &Repository) -> Result<(), String> {
        let index_path = Index::path(&repo);

        // TODO: can I do this without the nested map_err?
        serde_json::to_string(self).map_err(|e| e.to_string())
            .and_then(|contents| fs::write(index_path, contents).map_err(|e| e.to_string()))
    }

    // TODO: should 'path' functions move to file_utils?
    fn path(repo: &Repository) -> PathBuf {
        repo_file(&repo, vec!["index".to_string()], false)
            .and_then(|path_string| Ok(Path::new(&path_string).to_owned()))
            .unwrap()
    }
}

impl IndexEntry {

}