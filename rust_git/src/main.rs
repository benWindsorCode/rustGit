use rust_git::cli::Cli;
use rust_git::repository::Repository;

fn main() {
    let cli = Cli::new();
    cli.run();

    // let repo = Repository::find(String::from("C:\\Users\\benja\\Documents\\code\\my_git_test"), false).unwrap();
    // println!("{}\n{}\n{:?}", repo.worktree, repo.gitdir, repo.conf.contents);
}
