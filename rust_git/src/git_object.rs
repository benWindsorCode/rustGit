use bytes::Bytes;

pub trait GitWriteable<T> {
    fn serialize(&self, data: T);
    fn deserialize(&self, data: Bytes) -> T;
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
pub struct GitBlob {}