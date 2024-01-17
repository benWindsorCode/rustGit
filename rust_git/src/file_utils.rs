use std::fs::create_dir_all;
use std::path::{Path, PathBuf};
use crate::repository::Repository;

///
/// Given a repo and a vec of folders in that repo, create a path to those folders in the repo
/// within the gitdir of the rpo
///
/// ```
/// use rust_git::config::Config;
/// use rust_git::repository::Repository;
/// let repo = Repository { worktree: String::from(""), gitdir: String::from("git\\path"), conf: Config::new(String::from("dummy/path")) };
/// let path = vec![String::from("test"), String::from("test2")];
/// assert_eq!(rust_git::file_utils::repo_path(&repo, path), String::from("git\\path\\test\\test2"))
/// ```
///
pub fn repo_path(repository: &Repository, path: Vec<String>) -> String {
    let mut path_obj = PathBuf::from(&repository.gitdir);
    for item in path {
        path_obj.push(item)
    }

    String::from(path_obj.to_str().unwrap())
}

/// Given a repo and a path to a dir inside the gitdir, create the directory if it doesnt exist
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
    }

    Some(String::from(path_obj.to_str().unwrap()))
}

/// Given a repository and a path inside the gitdir, create the path to file if it doesnt exist
pub fn repo_file(repository: &Repository, path: Vec<String>, mkdir: bool) -> Result<String, String> {
    path.split_last().and_then(|(_, rest)| {
        repo_dir(&repository, rest.to_vec(), mkdir)
    }).and_then(|_| {
        Some(repo_path(&repository, path))
    }).ok_or("Failed to run repo_file".to_string())
}

#[cfg(test)]
mod test {
    use std::path::{Path, PathBuf};
    use tempdir::TempDir;
    use crate::file_utils::repo_path;
    use crate::repository::Repository;

    #[test]
    fn test_repo_path() {
        let tmp_dir = TempDir::new("dummy_repo").unwrap();
        let tmp_dir_string: String = tmp_dir.path().to_str().unwrap().into();

        // initialise an empty repo in the temp dir
        let repo = Repository::create(tmp_dir_string.clone());
        println!("Created test repo: {:?} in {:?}", repo, tmp_dir);
        assert!(repo.is_ok());

        let path = repo_path(&repo.unwrap(), vec!["first".to_string(), "second".to_string()]);

        let mut expected = PathBuf::from(tmp_dir_string);
        expected.push(Path::new(".git/first/second"));
        assert_eq!(Path::new(&path), expected.as_path())
    }
}