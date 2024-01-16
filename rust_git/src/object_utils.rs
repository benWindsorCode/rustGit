use std::collections::HashMap;
use std::fs;
use std::num::ParseIntError;
use std::path::{Path, PathBuf};
use bytes::{BufMut, Bytes, BytesMut};
use regex::Regex;
use sha1::{Digest, Sha1};
use crate::file_utils::repo_file;
use crate::git_object::{GitBlob, GitCommit, GitObject, GitTag, GitTree, GitWriteable};
use crate::refs::{Ref, RefType};
use crate::repository::Repository;

/// Parse a git object given the sha hash of the file
///
/// The parsing works as follows:
/// - the sha hash is split into [first 2 chars]:[rest of chars]
/// - the file lives inside .git/[first 2 chars]/[rest of chars]
///
/// Once loaded the contents of the file follows the pattern:
/// [format][space char][object size][null byte][data]
///
/// Where format is one of the types of GitObject e.g. Commit, Blob etc.
///
/// Once the type is determined we can deserialize the data into an instance of GitObject
/// via its GitWriteable trait implementation
pub fn object_read(repo: &Repository, sha: String) -> Result<GitObject, String> {
    let file = repo_file(&repo, vec![String::from("objects"), String::from(&sha[..2]), String::from(&sha[2..])], false)?;
    let path = Path::new(&file);

    if !path.is_file() {
        return Err("Directory not file".to_string());
    }

    let bytes = Bytes::from(fs::read(path).unwrap());

    let format_loc_index = bytes.iter().position(|&b| b == b' ').ok_or("Couldnt locate format locator byte")?;
    let format = &bytes.as_ref()[..format_loc_index];
    // Add 1 to strip out the space
    let rest = &bytes.as_ref()[format_loc_index+1..];

    let size_loc_index = rest.iter().position(|&b| b == b'\x00').ok_or("Couldnt locate size locator byte")?;

    // TODO: handle the unwrap here more elegantly?
    // Get the size as bytes, turn it into the right char, and parse this into a usize
    let size_raw = &rest.as_ref()[..size_loc_index];
    let size: usize = String::from_utf8(size_raw.to_vec()).unwrap().parse().or_else(|e: ParseIntError| Err(e.to_string()))?;

    // Add 1 to account for the null byte
    let data = &rest.as_ref()[size_loc_index+1..];

    if size != data.len() {
        return Err("Data did not pass size validation".to_string());
    }

    match std::str::from_utf8(format) {
        Ok("blob") => Ok(GitObject::Blob(GitBlob::deserialize(Bytes::from(data.to_owned())))),
        Ok("commit") => Ok(GitObject::Commit(GitCommit::deserialize(Bytes::from(data.to_owned())))),
        Ok("tree") => Ok(GitObject::Tree(GitTree::deserialize(Bytes::from(data.to_owned())))),
        Ok(other) => {
            // TODO: work out how to get the 'other' string into the Err message without issues of 'value referencing data owned by the current function'
            println!("ERROR: unable to parse format: {}", other);
            Err("Unable to parse format, see logs for details".to_string())
        },
        _ => Err("Unable to parse format".to_string())
    }
}

/// Given a GitObject, write it into the repo and return its sha hash
///
/// The object follows the pattern:
/// [format][space char][object size][null byte][data]
///
/// Can be undone via the object_read function
pub fn object_write(obj: GitObject, repo_option: Option<&Repository>) -> Result<String, String> {
    // TODO: I could definitely have done this more nicely, in particular by actioning the other TODO in git_object.rs
    //       about not having the 'inner types' of the GitObject enum and directly implementing the below traits
    //       on the GitObject itself
    let data = match &obj {
        GitObject::Blob(blob) => blob.serialize(),
        GitObject::Commit(commit) => commit.serialize(),
        GitObject::Tree(tree) => tree.serialize(),
        GitObject::Tag(tag) => tag.serialize(),
    };

    let format = match &obj  {
        GitObject::Blob(_) => GitBlob::format_name(),
        GitObject::Commit(_) => GitCommit::format_name(),
        GitObject::Tree(_) => GitTree::format_name(),
        GitObject::Tag(_) => GitTag::format_name(),
    };

    // todo: if we reserve with BytesMut::with_capacity(n) upfront we get better efficiency
    let mut output_data = BytesMut::new();
    output_data.put(Bytes::from(format));
    output_data.put_u8(b' ');
    output_data.put(Bytes::from(data.len().to_string()));
    output_data.put_u8(b'\x00');
    output_data.put(data);

    let mut hasher = Sha1::new();
    hasher.update(&output_data);
    let sha = format!("{:x}", hasher.finalize());

    if let Some(repo) = repo_option {
        let path = repo_file(&repo, vec![String::from("objects"), String::from(&sha[..2]), String::from(&sha[2..])], true)?;
        let path_obj = Path::new(&path);

        if !path_obj.exists() {
            fs::write(path, output_data).or_else(|e| Err(e.to_string()))?;
        }

    }

    Ok(sha)
}

// TODO: format is allowed to be blank, make it optional?
pub fn object_find(repo: &Repository, name: &String, format: &String, follow: bool) -> String {
    println!("Running object_find for {:?}, {} {} {}", repo, name, format, follow);
    name.to_owned()
}

/// A watered down version of the full git resolution algorithm
///
/// Including support for 'short hashes', to reference a hash by the first 6 chars of the hash
///
/// From the object_find chapter: https://wyag.thb.lt/#object_find
fn object_resolve(repo: &Repository, name: String) -> Vec<String> {
    let mut candidates = Vec::new();


    let trimmed = name.trim().to_owned();
    if trimmed.is_empty() {
        return candidates;
    }

    if trimmed == "HEAD" {
        let resolved = Ref::new("HEAD".to_string()).fully_resolve(&repo);

        let head_ref = match resolved {
            RefType::Direct(head_ref) => head_ref,
            _ => panic!("Couldnt resolve ref to head")
        };

        candidates.push(head_ref);
        return candidates;
    }

    // If its a hex string, try to resolve
    let hex_string_re = Regex::new(r"^[0-9A-Fa-f]{4,40}$").unwrap();
    if hex_string_re.is_match(trimmed.as_str()) {
        todo!("Implement hex string resolution");
    }

    todo!("Implement reference resolution");

    candidates
}

pub fn tree_to_dict(repo: &Repository, name: &String, prefix: Option<&'static str>) -> HashMap<String, String> {
    let mut ret = HashMap::new();

    let tree_sha = object_find(&repo, name, &"tree".to_string(), true);
    let tree = match object_read(&repo, tree_sha) {
        Ok(GitObject::Tree(obj)) => obj,
        _ => return ret
    };

    for leaf in tree.items {
        let mut path_items = vec![];
        if let Some(prefix_str) = prefix {
            path_items.push(prefix_str);
        }
        path_items.push(&leaf.path);

        let mut path = PathBuf::new();
        for item in path_items {
            path.push(item);
        }

        // TODO: add subtree support per here https://wyag.thb.lt/#org154db98

        ret.insert(path.as_path().to_str().unwrap().into(), leaf.sha);
    }

    ret
}