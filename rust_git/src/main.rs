use bytes::Bytes;
use rust_git::cli::Cli;
use rust_git::git_object::{GitBlob, GitObject, GitWriteable};
use rust_git::object_utils::{object_read, object_write};
use rust_git::repository::Repository;

fn main() {
    let cli = Cli::new();
    // cli.run();

    let repo = Repository::find(String::from("C:\\Users\\benja\\Documents\\code\\my_git_test"), false).unwrap();
    let blob = GitBlob::deserialize(Bytes::from("test123"));
    let obj = GitObject::Blob(blob);

    let res = object_read(&repo, String::from("0c759a44455ebbc12b9f5569692f515b38271896")).unwrap();
    println!("{:?}", res);
    // object_write(obj, Some(repo));
    // println!("{}\n{}\n{:?}", repo.worktree, repo.gitdir, repo.conf.contents);
}
