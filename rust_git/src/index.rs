use std::fs;
use std::path::{Path, PathBuf};
use std::time::SystemTime;
use serde::{Deserialize, Serialize};
use crate::file_utils::repo_file;
use crate::repository::Repository;

#[derive(Serialize, Deserialize, Debug)]
pub struct Index {
    version: i32,
    pub entries: Vec<IndexEntry>
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct IndexEntry {
    // TODO: is this the right way to store time?
    pub time: SystemTime,
    pub mtime: SystemTime,
    pub dev: String,
    // TODO: could this be u32?
    pub ino: i32,
    pub model_type: ModelType,
    pub model_perms: i32,
    pub uid: i32,
    pub gid: i32,
    // Size of the object in bytes
    pub fsize: u64,
    pub sha: String,
    pub flag_assume_valid: bool,
    pub flag_stage: bool,
    // Full path of the object
    pub name: String
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum ModelType {
    // b1000
    Regular,
    // b1010
    Symlink,
    // b1110
    Gitlink
}

impl Index {
    pub fn new() -> Self {
        Index { version: 2, entries: Vec::new() }
    }

    pub fn add_entry(&mut self, entry: IndexEntry) {
        self.entries.push(entry);
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

    pub fn get_gitignore(&self) -> Option<IndexEntry> {
        for entry in &self.entries {
            if entry.name.contains(".gitignore") {
                return Some(entry.clone());
            }
        }

        None
    }
}

impl IndexEntry {
    pub fn new(sha: String, path: String) -> Self {
        let metadata = fs::metadata(Path::new(&path)).unwrap();

        // For some definitions of these fields from the tutorial see https://www.gnu.org/software/libc/manual/html_node/Attribute-Meanings.html
        IndexEntry {
            time: metadata.created().unwrap(),
            mtime: metadata.modified().unwrap(),
            // TODO: platform specific https://doc.rust-lang.org/std/os/linux/fs/trait.MetadataExt.html#tymethod.st_dev
            dev: "".to_string(),
            // TODO: platform specific https://doc.rust-lang.org/std/os/linux/fs/trait.MetadataExt.html#tymethod.st_ino
            ino: 0,
            model_type: ModelType::Regular,
            model_perms: 0o644,
            // TODO: platform specific https://doc.rust-lang.org/std/os/linux/fs/trait.MetadataExt.html#tymethod.st_uid
            uid: 0,
            // TODO: platform specific https://doc.rust-lang.org/std/os/linux/fs/trait.MetadataExt.html#tymethod.st_gid
            gid: 0,
            fsize: metadata.len(),
            sha,
            flag_assume_valid: false,
            flag_stage: false,
            name: path,
        }
    }
}