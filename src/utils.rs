use std::path::PathBuf;
use crate::repository::Repository;

///
/// Given a repo and a vec of folders in that repo, create a path to those folders in the repo
/// within the gitdir of the rpo
///
/// ```
/// use rustGit::repository::Repository;
/// let repo = Repository { worktree: String::from(""), gitdir: String::from("git\\path") };
/// let path = vec![String::from("test"), String::from("test2")];
/// assert_eq!(rustGit::utils::repo_path(repo, path), String::from("git\\path\\test\\test2"))
/// ```
///
pub fn repo_path(repository: Repository, path: Vec<String>) -> String {
    let mut path_obj = PathBuf::from(&repository.gitdir);
    for item in path {
        path_obj.push(item)
    }

    String::from(path_obj.to_str().unwrap())
}