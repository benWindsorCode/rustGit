use std::fs;
use std::path::PathBuf;
use std::time::SystemTime;
use bytes::Bytes;
use rust_git::file_utils::repo_path;
use rust_git::git_object::{GitBlob, GitObject};
use rust_git::index::{Index, IndexEntry, ModelType};
use rust_git::object_utils::object_write;
use rust_git::repository::Repository;

fn main() {
    let repo = Repository::find(String::from("."), true).unwrap();

    let gitignore_data = "#my comment\n*.iml\n!test".to_string();

    let mut gitignore_path = PathBuf::from(&repo.worktree);
    gitignore_path.push(".gitignore");
    fs::write(gitignore_path, gitignore_data.clone()).unwrap();

    let gitignore = GitBlob { data: Some(Bytes::from(gitignore_data)) };
    let gitignore_sha = object_write(GitObject::Blob(gitignore), Some(&repo)).unwrap();


    println!("Sucesfully wrote .gitignore with hash {}", gitignore_sha);
    let gitignore_entry = IndexEntry {
        time: SystemTime::now(),
        mtime: SystemTime::now(),
        dev: "".to_string(),
        ino: 0,
        model_type: ModelType::Regular,
        model_perms: 0,
        uid: 0,
        gid: 0,
        fsize: 0,
        sha: gitignore_sha,
        flag_assume_valid: false,
        flag_stage: false,
        name: "C:\\Users\\benja\\Documents\\code\\my_git_test\\.gitignore".to_string()
    };

    let mut index = Index::new();
    index.add_entry(gitignore_entry);
    index.write(&repo).unwrap();

    let gitignore_found = index.get_gitignore();
    println!("Found gitignore: {:?}", gitignore_found);
}