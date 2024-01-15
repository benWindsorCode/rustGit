use std::fs;
use std::fs::create_dir_all;
use std::path::{Path, PathBuf};
use bytes::Bytes;
use serde::{Deserialize, Serialize};
use crate::key_value_list_message::{KeyValuePairEntry, KeyValuePairKey, KeyValuePairList};
use crate::object_utils::object_read;
use crate::repository::Repository;

/// The GitWriteable trait represents an object which can be serialised and deserialised.
/// This could have been avoided by directly using serdes traits but it was more in the spirit of the tutorial
/// to create our own format for the key value pair structure of the commits even though ultimately it was not needed
pub trait GitWriteable<T: GitWriteable<T>> {

    /// Always create a new object, if provided data deserialise it else just make an empty object
    fn from(data: Option<Bytes>) -> T {
        match data {
            None => T::new(),
            Some(contents) => T::deserialize(contents)
        }
    }

    /// Create an empty version of the object
    fn new() -> T;

    fn format_name() -> String;

    /// Take an object and turn it into bytes
    fn serialize(&self) -> Bytes;

    /// Take in data and return an object of the right type
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

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct GitTree {
    items: Vec<GitLeaf>
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct GitLeaf {
    pub mode: String,
    pub path: String,
    pub sha: String,
    pub sort_key: String
}

#[derive(Debug)]

pub struct GitTag {
    data: GitTagData
}

#[derive(Debug)]

enum GitTagData {
    // Makes a commit with this data, fetches the sha of that and then creates a ref to that sha
    Object {
        object: String,
        tag: String,
        tagger: String,
    },
    // Ultimately just a ref to a commit/tree/blob
    Lightweight(String)
}

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

// TODO: create a ::new() function which automatically populates the leaf key on add
impl GitLeaf {
}

impl GitTree {
    pub fn checkout(&self, repo: &Repository, path: &Path) {
        for leaf in &self.items {
            let mut base_path = PathBuf::from(path);
            base_path.push(&leaf.path);

            let success= object_read(&repo, leaf.sha.clone()).map(|obj| {
                match obj {
                    GitObject::Tree(tree) => {
                        create_dir_all(&base_path)
                            .and_then(|_| Ok(tree.checkout(&repo, &base_path)))
                            .or_else(|_| Err("Unable to process tree checkout"))
                    },
                    GitObject::Blob(blob) => {
                        let _ = &base_path.parent()
                            .and_then(|parent| create_dir_all(parent).ok());

                        blob.data
                            .and_then(|data| fs::write(&base_path, data).ok());

                        Ok(())
                    },
                    _ => Err("Unsupported object found in checkout")
                }
            }).is_ok();

            if success {
                println!("Sucesfully checkout file to {:?}", &base_path);
            } else {
                println!("ERROR: could not checkout file to {:?}", &base_path);
            }
        }
    }

    // TODO: Add should sort by the leaf key
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