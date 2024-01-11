use bytes::Bytes;
use serde::{Deserialize, Serialize};
use crate::key_value_list_message::{KeyValuePairList};

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
    mode: String,
    path: String,
    sha: String,
    sort_key: String
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