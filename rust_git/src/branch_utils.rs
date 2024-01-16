use std::fs;
use crate::file_utils::repo_file;
use crate::repository::Repository;

pub fn branch_get_active(repo: &Repository) -> Option<String> {
    get_head_contents(&repo).and_then(|head_contents| {
        if head_contents.starts_with("ref: refs/heads/") {
            head_contents.strip_prefix("ref: refs/heads/")
                .ok_or("Unable to remove head prefix".to_owned())
                .map(|contents| contents.to_owned())
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