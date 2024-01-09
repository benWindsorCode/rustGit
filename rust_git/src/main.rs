use bytes::Bytes;
use rust_git::cli::Cli;
use rust_git::git_object::{GitBlob, GitObject, GitWriteable};
use rust_git::object_utils::{object_read, object_write};
use rust_git::repository::Repository;

fn main() {
    use rust_git::key_value_list_message::{KeyValuePairEntry, KeyValuePairList};
    let input = Bytes::from("firstkey firstvalue\n continuation of value\n further continuation\nsecondkey secondvalue\n");
    let output = KeyValuePairList::from(input).unwrap();
    // let cli = Cli::new();
    // cli.run();

    // Repository::find(String::from("."), true);
    // let repo = Repository::find(String::from("C:\\Users\\benja\\Documents\\code\\my_git_test"), false).unwrap();
    // let blob = GitBlob::deserialize(Bytes::from("Ben and Ella 2"));
    // let obj = GitObject::Blob(blob);

    // let res = object_read(&repo, String::from("ebf769b790cda0ac191040a3e144dd776cc27194")).unwrap();
    // println!("{:?}", res);
    // object_write(obj, Some(repo));
    // println!("{}\n{}\n{:?}", repo.worktree, repo.gitdir, repo.conf.contents);
}
