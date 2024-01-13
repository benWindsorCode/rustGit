use rust_git::repository::Repository;

fn main() {
    let repo_path = "C:\\Users\\benja\\Documents\\code\\my_git_test".to_string();
    Repository::create(repo_path).unwrap();
}