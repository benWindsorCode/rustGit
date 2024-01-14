use rust_git::refs::{Ref, RefType};
use rust_git::repository::Repository;

fn main() {
    let repo = Repository::find(String::from("."), true).unwrap();
    println!("Found repo: {:?}", repo);

    // Create a chain of refs: refs/heads/other -> refs/heads/main -> some_test_hash

    let mut reference = Ref::new("refs/heads/main".to_string());
    reference.add_target(RefType::Direct("some_test_hash".to_string()));
    reference.write(&repo).unwrap();

    let mut reference_2 = Ref::new("refs/heads/other".to_string());
    reference_2.add_target(RefType::Indirect("refs/heads/main".to_string()));
    reference_2.write(&repo).unwrap();

    let resolved = reference.fully_resolve(&repo);
    println!("reference resolved to: {:?}", resolved);

    let resolved_2 = reference_2.fully_resolve(&repo);
    println!("reference_2 resolved to: {:?}", resolved_2);
}