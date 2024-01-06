pub trait GitWriteable {
    fn serialize(&self);
    fn deserialize(&self);
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