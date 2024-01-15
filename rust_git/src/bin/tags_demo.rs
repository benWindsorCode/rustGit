use rust_git::git_object::GitTag;
use rust_git::repository::Repository;

fn main() {
    let repo = Repository::find(String::from("."), true).unwrap();

    // A lightweight tag is just a reference to an object
    let lightweight_tag = GitTag::new_lightweight("my_tag_name".to_string(), "some_object_hash".to_string(), &repo);
    lightweight_tag.write(&repo).unwrap();

    // A tag object is a reference to an actual object with more data about the thing thats tagged
    let object_tag = GitTag::new_object("my_tag_object_name".to_string(), "some_other_hash".to_string(), &repo);
    object_tag.write(&repo).unwrap();
}