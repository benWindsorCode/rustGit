use std::fs;
use std::path::Path;
use bytes::{BufMut, Bytes, BytesMut};
use sha1::{Digest, Sha1};
use crate::file_utils::repo_file;
use crate::git_object::{GitBlob, GitCommit, GitObject, GitWriteable};
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
pub fn object_read(repo: &Repository, sha: String) -> Result<GitObject, &'static str> {
    let file = repo_file(&repo, vec![String::from("objects"), String::from(&sha[..2]), String::from(&sha[2..])], false)?;
    let path = Path::new(&file);

    if !path.is_file() {
        return Err("Directory not file");
    }

    let bytes = Bytes::from(fs::read(path).unwrap());

    let format_loc_index = bytes.iter().position(|&b| b == b' ').ok_or("Couldnt locate format locator byte")?;
    let format = &bytes.as_ref()[..format_loc_index];
    let rest = &bytes.as_ref()[format_loc_index..];

    let size_loc_index = rest.iter().position(|&b| b == b'\x00').ok_or("Couldnt locate size locator byte")?;
    let size = &rest.as_ref()[..size_loc_index];
    let size = String::from_utf8(size.to_vec()).map(|size_str| size_str.parse::<usize>().unwrap()).unwrap();
    let data = &rest.as_ref()[size_loc_index..];

    if size != data.len() {
        return Err("Data did not pass size validation");
    }

    match std::str::from_utf8(format) {
        Ok("blob") => Ok(GitObject::Blob(GitBlob::deserialize(Bytes::from(data.to_owned())))),
        _ => Err("Unrecognised format")
    }
}

/// Given a GitObject, write it into the repo and return its sha hash
pub fn object_write(obj: GitObject, repo_option: Option<Repository>) -> Result<String, &'static str> {
    let data = match &obj {
        GitObject::Blob(blob) => blob.serialize(),
        _ => panic!("type unsupported")
    };

    let format = match &obj  {
        GitObject::Blob(_) => GitBlob::format_name(),
        _ => panic!("type unsupported")
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
            fs::write(path, output_data);
        }

    }

    Ok(sha)
}