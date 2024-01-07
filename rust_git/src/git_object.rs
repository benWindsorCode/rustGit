use bytes::Bytes;

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

pub enum GitObject {
    Commit(GitCommit),
    Tree(GitTree),
    Tag(GitTag),
    Blob(GitBlob)
}

pub struct GitCommit {}
pub struct GitTree {}
pub struct GitTag {}
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