use std::fs;
use std::fs::create_dir_all;
use std::path::{Path, PathBuf};
use bytes::Bytes;
use serde::{Deserialize, Serialize};
use crate::key_value_list_message::{KeyValuePairEntry, KeyValuePairKey, KeyValuePairList};
use crate::object_utils::object_read;
use crate::repository::Repository;

pub trait GitWriteable<T: GitWriteable<T>> {

    // Always create a new object, if provided data deserialise it else just make an empty object
    fn from(data: Option<Bytes>) -> T {
        match data {
            None => T::new(),
            Some(contents) => T::deserialize(contents)
        }
    }

    // Create an empty version of the object
    fn new() -> T;

    fn format_name() -> String;

    // Take an object and turn it into bytes
    // TODO: feels like T should be here somewhere? but I guess its handled by the generic definition as T is GitWriteable<T> recursively
    fn serialize(&self) -> Bytes;

    // Take in data and return an object of the right type
    fn deserialize(data: Bytes) -> T;
}

#[derive(Debug)]
pub enum GitObject {
    Commit(GitCommit),
    Tree(GitTree),
    Tag(GitTag),
    Blob(GitBlob)
}

#[derive(Debug)]
pub struct GitCommit {
    pub data: KeyValuePairList
}

#[derive(Serialize, Deserialize, Debug)]
pub struct GitTree {
    items: Vec<GitLeaf>
}

#[derive(Serialize, Deserialize, Debug)]
pub struct GitLeaf {
    pub mode: String,
    pub path: String,
    pub sha: String,
    pub sort_key: String
}

#[derive(Debug)]

pub struct GitTag {}

#[derive(Debug)]

pub struct GitBlob {
    pub data: Option<Bytes>
}

impl GitWriteable<GitBlob> for GitBlob {
    fn new() -> GitBlob {
        GitBlob { data: None }
    }

    fn format_name() -> String {
        String::from("blob")
    }

    fn serialize(&self) -> Bytes {
        self.data.clone().unwrap()
    }

    fn deserialize(data: Bytes) -> GitBlob {
        GitBlob { data: Some(data) }
    }
}

impl GitWriteable<GitCommit> for GitCommit {
    fn new() -> GitCommit {
        GitCommit { data: KeyValuePairList::new() }
    }

    fn format_name() -> String {
        String::from("commit")
    }

    fn serialize(&self) -> Bytes {
        self.data.into_bytes()
    }

    fn deserialize(data: Bytes) -> GitCommit {
        GitCommit { data: KeyValuePairList::from(data).unwrap() }
    }
}

impl GitCommit {
    pub fn get_tree_string(&self) -> Option<String> {
        let tree_entry = match self.data.get(KeyValuePairKey::Key("tree".to_string())) {
            None => return None,
            Some(entry) => entry
        };

        match tree_entry {
            KeyValuePairEntry::Singleton(tree) => return Some(String::from_utf8(tree.to_vec()).unwrap()),
            KeyValuePairEntry::List(_) => panic!("Tree of type list not supported")
        }
    }

    pub fn get_and_read_tree(&self, repo: &Repository) -> Result<GitTree, &'static str> {
        self.get_tree_string()
            .ok_or("Tree not found")
            .and_then(|tree_hash| object_read(&repo, tree_hash) )
            .and_then(|obj| match obj {
                GitObject::Tree(tree) => Ok(tree),
                _ => Err("Non tree object found")
            })
    }
}

impl GitLeaf {
}

impl GitTree {
    pub fn checkout(&self, repo: &Repository, path: &Path) {
        for leaf in &self.items {
            let mut base_path = PathBuf::from(path.clone());
            base_path.push(&leaf.path);

            let success= object_read(&repo, leaf.sha.clone()).map(|obj| {
                match obj {
                    GitObject::Tree(tree) => {
                        create_dir_all(&base_path);
                        tree.checkout(&repo, &base_path);
                    },
                    GitObject::Blob(blob) => {
                        create_dir_all(&base_path.parent().unwrap());
                        fs::write(&base_path, blob.data.unwrap()).unwrap();
                    },
                    _ => {}
                }
            }).is_ok();

            if success {
                println!("Sucesfully checkout file to {:?}", &base_path);
            } else {
                println!("ERROR: could not checkout file to {:?}", &base_path);
            }
        }
    }

    pub fn add(&mut self, git_leaf: GitLeaf) {
        self.items.push(git_leaf);
    }
}

impl GitWriteable<GitTree> for GitTree {
    fn new() -> GitTree {
        GitTree { items: Vec::new() }
    }

    fn format_name() -> String {
        String::from("tree")
    }

    fn serialize(&self) -> Bytes {
        // TODO: sort by leaf keys before serialize
        Bytes::from(serde_json::to_string(self).unwrap())
    }

    fn deserialize(data: Bytes) -> GitTree {
        serde_json::from_str(&String::from_utf8(data.to_vec()).unwrap()).unwrap()
    }
}