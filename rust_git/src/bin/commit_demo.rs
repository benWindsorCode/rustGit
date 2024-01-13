use bytes::Bytes;
use rust_git::git_object::{GitBlob, GitCommit, GitLeaf, GitObject, GitTree, GitWriteable};
use rust_git::key_value_list_message::KeyValuePairList;
use rust_git::object_utils::object_write;
use rust_git::repository::Repository;

fn main() {
    let repo = Repository::find(String::from("C:\\Users\\benja\\Documents\\code\\my_git_test"), false).unwrap();
    println!("Repo opened - worktree '{}'\ngitdir '{}'\nconfig '{:?}'", repo.worktree, repo.gitdir, repo.conf.contents);

    let blob_1 = GitBlob::deserialize(Bytes::from("First file of commit"));
    let obj_1 = GitObject::Blob(blob_1);
    let sha_1 = object_write(obj_1, Some(&repo));
    println!("Object 1 written to {:?}", sha_1);

    let blob_2 = GitBlob::deserialize(Bytes::from("Second file of commit"));
    let obj_2 = GitObject::Blob(blob_2);
    let sha_2 = object_write(obj_2, Some(&repo));
    println!("Object 2 written to {:?}", sha_2);

    let leaf_1 = GitLeaf {
        mode: "".to_string(),
        path: "./file1.txt".to_string(),
        sha: sha_1.unwrap(),
        sort_key: "".to_string(),
    };

    let leaf_2 = GitLeaf {
        mode: "".to_string(),
        path: "./test/file2.txt".to_string(),
        sha: sha_2.unwrap(),
        sort_key: "".to_string(),
    };

    let mut tree = GitTree::new();
    tree.add(leaf_1);
    tree.add(leaf_2);

    let tree_hash = object_write(GitObject::Tree(tree.clone()), Some(&repo)).unwrap();
    println!("Saved tree to: {}", tree_hash.clone());

    let mut commit_data = KeyValuePairList::new();
    commit_data.insert_pair("tree".to_string(), Bytes::from(tree_hash));
    commit_data.insert_contents(Bytes::from("My first commit message"));

    let commit = GitCommit { data: commit_data };
    let commit_hash = object_write(GitObject::Commit(commit), Some(&repo)).unwrap();
    println!("Commit saved to: {}", commit_hash.clone());
}