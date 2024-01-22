use std::fs;
use crate::file_utils::repo_file;
use crate::repository::Repository;

const HEAD_REF: &str = "ref: refs/heads/";

pub fn branch_get_active(repo: &Repository) -> Option<String> {
    get_head_contents(&repo).and_then(|head_contents| {
        if head_contents.starts_with(HEAD_REF) {
            head_contents.strip_prefix(HEAD_REF)
                .ok_or("Unable to remove head prefix".to_owned())
                .map(|contents| contents.trim().to_owned())
        } else {
            Err("Head ref not found".to_owned())
        }
    }).ok()
}

fn get_head_contents(repo: &Repository) -> Result<String, String> {
    repo_file(&repo, vec!["HEAD".to_string()], false)
        .map_err(|e| e.to_owned())
        .and_then(|path| fs::read_to_string(path).map_err(|e| format!("{}", e)))
}

#[cfg(test)]
mod test {
    use tempdir::TempDir;
    use crate::branch_utils::branch_get_active;
    use crate::repository::Repository;

    #[test]
    fn test_branch_get_active() {
        let tmp_dir = TempDir::new("dummy_repo").unwrap();
        let tmp_dir_string: String = tmp_dir.path().to_str().unwrap().into();

        // initialise an empty repo in the temp dir
        let repo = Repository::create(tmp_dir_string.clone());
        println!("Created test repo: {:?} in {:?}", repo, tmp_dir);
        assert!(repo.is_ok());

        let branch = branch_get_active(&repo.unwrap());
        assert_eq!(branch, Some("master".to_string()));
    }
}