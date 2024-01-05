use std::fs::create_dir_all;
use std::path::{Path, PathBuf};
use crate::repository::Repository;

///
/// Given a repo and a vec of folders in that repo, create a path to those folders in the repo
/// within the gitdir of the rpo
///
/// ```
/// use rustGit::repository::Repository;
/// let repo = Repository { worktree: String::from(""), gitdir: String::from("git\\path") };
/// let path = vec![String::from("test"), String::from("test2")];
/// assert_eq!(rustGit::utils::repo_path(&repo, path), String::from("git\\path\\test\\test2"))
/// ```
///
pub fn repo_path(repository: &Repository, path: Vec<String>) -> String {
    let mut path_obj = PathBuf::from(&repository.gitdir);
    for item in path {
        path_obj.push(item)
    }

    String::from(path_obj.to_str().unwrap())
}

///
/// Given a repo and a path to a dir inside the gitdir, create the directory if it doesnt exist
///
pub fn repo_dir(repository: &Repository, path: Vec<String>, mkdir: bool) -> Option<String> {
    let path_name = repo_path(repository, path);

    let path_obj = Path::new(&path_name);

    if path_obj.exists() {
        if path_obj.is_dir() {
            return Some(String::from(path_obj.to_str().unwrap()));
        } else {
            panic!("{} is not a directory", path_name.clone());
        }
    }

    if mkdir {
        create_dir_all(path_obj).unwrap();
        return Some(String::from(path_obj.to_str().unwrap()));
    }

    None
}

///
/// Given a repository and a path inside the gitdir, create the file if it doesnt exist
pub fn repo_file(repository: &Repository, path: Vec<String>, mkdir: bool) {

}