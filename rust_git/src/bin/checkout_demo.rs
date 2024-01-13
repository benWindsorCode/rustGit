use rust_git::cli::Cli;
use rust_git::repository::Repository;

fn main() {
    Repository::find(String::from("."), true);
    let repo = Repository::find(String::from("C:\\Users\\benja\\Documents\\code\\my_git_test"), false).unwrap();
    println!("Repo opened - worktree '{}'\ngitdir '{}'\nconfig '{:?}'", repo.worktree, repo.gitdir, repo.conf.contents);

    let cli = Cli::new();

    // Get this has from running commit_demo.rs to create a commit
    let commit = "21d9aba10852b0e7efff48fc30703c34a03e178c".to_string();
    let path = "C:\\Users\\benja\\Documents\\code\\my_git_test\\checkout".to_string();
    cli.process_checkout(&commit, &path);
}