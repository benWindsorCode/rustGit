use rust_git::ignore::Ignore;
use rust_git::repository::Repository;

fn main() {
    // N.B.: run the index_demo to create the gitignore and an index in the repo first for it

    let repo = Repository::find(String::from("."), true).unwrap();

    let ignore = Ignore::read(&repo);

    println!("Ignore file loaded: {:?}", ignore);
}