use std::fs;
use std::path::Path;
use serde::{Deserialize, Serialize};
use walkdir::WalkDir;
use crate::file_utils::{repo_dir, repo_file};
use crate::repository::Repository;

#[derive(Serialize, Deserialize, Debug)]
pub struct Ref {
    // name of the reference e.g. refs/feature/test123 or main
    // ultimately ends up as a path to a file inside the .git/refs directory
    pub name: String,
    pub target: Option<RefType>
}

#[derive(Serialize, Deserialize, Debug, PartialEq)]
pub enum RefType {

    // The hash of an object
    Direct(String),

    // The name of another ref itself
    Indirect(String),

    // The following is taken directly from https://wyag.thb.lt/#cmd-show-ref:
    //      Sometimes, an indirect reference may be broken.  This is normal
    //      in one specific case: we're looking for HEAD on a new repository
    //      with no commits.  In that case, .git/HEAD points to "ref:
    //      refs/heads/main", but .git/refs/heads/main doesn't exist yet
    //      (since there's no commit for it to refer to)
    //
    // Hence we need some way to represent a reference that doesnt exist
    Broken
}

impl Ref {
    pub fn new(name: String) -> Self {
        Ref { name, target: None }
    }

    // TODO: this should return a Result<Ref, String> and make the error handling cleaner
    pub fn from_file(name: String, repo: &Repository) -> Self {
        let path = repo_file(repo, vec![name.clone()], false).map_err(|e| e.to_string()).unwrap();
        fs::read(path).and_then(|data| Ok(serde_json::from_slice(&data).unwrap())).unwrap()
    }

    pub fn all_refs(repo: &Repository) -> Vec<Ref> {
        let path = repo_dir(&repo, vec!["refs".to_string()], false).unwrap();

        let mut result = Vec::new();
        for file in WalkDir::new(path).into_iter().filter_map(|file| file.ok()) {
            if !file.metadata().unwrap().is_file() {
                continue;
            }

            // TODO: load the ref file via serde so that the target is populated too rather than just the name
            file.path().strip_prefix(Path::new(&repo.gitdir))
                .and_then(|path| {
                    let new_ref = Ref::from_file(path.to_str().unwrap().to_string(), &repo);
                    result.push(new_ref);
                    Ok(())
                }).unwrap();
        }

        result
    }

    pub fn add_target(&mut self, target: RefType) {
        match target {
            RefType::Indirect(indirect_name) if indirect_name == self.name => panic!("Cannot have a ref be an indirect ref to itself"),
            _ => {}
        }

        self.target = Some(target);
    }

    pub fn is_indirect_ref(&self) -> bool {
        match self.target {
            Some(RefType::Indirect(_)) => true,
            _ => false
        }
    }

    pub fn is_broken_or_empty_ref(&self) -> bool {
        match self.target {
            Some(RefType::Broken) | None => true,
            _ => false
        }
    }

    pub fn write(&self, repo: &Repository) -> Result<(), String> {
        if self.is_broken_or_empty_ref() {
            return Err("Cant write a file without a ref".to_string());
        }

        let path = repo_file(repo, vec![self.name.clone()], false).map_err(|e| e.to_string())?;
        println!("Writing ref to: {:?}", path);
        serde_json::to_string(self)
            .map_err(|e| e.to_string())
            .and_then(|data| fs::write(path, data).or_else(|e| Err(e.to_string())))
    }

    /// Given a reference, start with its name and resolve away any Indirect references to
    /// produce either a RefType::Broken or a RefType::Direct
    pub fn fully_resolve(&self, repo: &Repository) -> RefType {
        let name_to_resolve = self.name.to_owned();

        match Ref::resolve_inner(name_to_resolve, repo) {
            RefType::Indirect(ref_name) => {
                let ref_chain = Ref::new(ref_name);
                ref_chain.fully_resolve(repo)
            },
            other @ _ => other
        }
    }

    fn resolve_inner(name: String, repo: &Repository) -> RefType {
        let path = repo_file(repo, vec![name], false)
            .and_then(|path_str| Ok(Path::new(&path_str).to_owned()))
            .unwrap();

        if !path.is_file() {
            return RefType::Broken;
        }

        // TODO: a nicer way to chain these together? the two functions return different error types
        //       could I map_err and turn them both to Strings? something like .map_err(|e| e.into())?
        let read_ref: Ref = fs::read(path)
            .and_then(|data| Ok(String::from_utf8(data).unwrap()))
            .and_then(|data_str| Ok(serde_json::from_str(&data_str).unwrap()))
            .unwrap();

        return read_ref.target.unwrap()
    }
}


#[cfg(test)]
mod tests {
    use bytes::Bytes;
    use tempdir::TempDir;
    use crate::git_object::{GitBlob, GitObject, GitWriteable};
    use crate::object_utils::object_write;
    use crate::refs::{Ref, RefType};
    use crate::repository::Repository;

    #[test]
    fn ref_create_and_resolve() {
        let tmp_dir = TempDir::new("dummy_repo").unwrap();
        let tmp_dir_string: String = tmp_dir.path().to_str().unwrap().into();

        // initialise an empty repo in the temp dir
        let repo = Repository::create(tmp_dir_string.clone()).unwrap();

        let blob_1 = GitBlob::deserialize(Bytes::from("First file of commit"));
        let obj_1 = GitObject::Blob(blob_1);
        let sha_1 = object_write(obj_1, Some(&repo)).unwrap();

        let mut reference = Ref::new("refs/heads/main".to_string());
        reference.add_target(RefType::Direct(sha_1.to_string()));

        let ref_write = reference.write(&repo);
        println!("ref write result: {:?}", ref_write);
        assert!(ref_write.is_ok());

        // Now resolve the reference
        let reference_resolve = Ref::new("refs/heads/main".to_string());
        let resolution = reference_resolve.fully_resolve(&repo);
        assert_eq!(resolution, RefType::Direct(sha_1.clone()));

        // create an indirect reference to the original reference
        let mut reference_indirect = Ref::new("refs/heads/other".to_string());
        reference_indirect.add_target(RefType::Indirect("refs/heads/main".to_string()));
        let ref_write_indirect = reference_indirect.write(&repo);
        assert!(ref_write_indirect.is_ok());

        // We now resolve the indirect reference:
        //        refs/heads/other -> refs/heads/main -> blob_1 hash from above
        let resolution_indirect = reference_indirect.fully_resolve(&repo);
        assert_eq!(resolution_indirect, RefType::Direct(sha_1.clone()));
    }
}