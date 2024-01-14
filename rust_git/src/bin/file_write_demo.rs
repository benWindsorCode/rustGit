use bytes::Bytes;
use rust_git::git_object::{GitBlob, GitObject, GitWriteable};
use rust_git::object_utils::object_write;
use rust_git::repository::Repository;

fn main() {
    Repository::find(String::from("."), true);
    let repo = Repository::find(String::from("C:\\Users\\benja\\Documents\\code\\my_git_test"), false).unwrap();
    println!("Repo opened - worktree '{}'\ngitdir '{}'\nconfig '{:?}'", repo.worktree, repo.gitdir, repo.conf.contents);

    let blob = GitBlob::deserialize(Bytes::from("This is some test data here"));
    let obj = GitObject::Blob(blob);

    let sha = object_write(obj, Some(&repo));
    println!("Object written to {:?}", sha);

}