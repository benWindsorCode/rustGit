use std::fs::{canonicalize, create_dir_all};
use std::path::Path;
use clap::{Parser, Subcommand};
use crate::branch_utils::branch_get_active;
use crate::git_object::GitObject::Commit;
use crate::git_object::{GitObject, GitTag};
use crate::ignore::Ignore;
use crate::index::Index;
use crate::object_utils::{object_find, object_read, object_write, tree_to_dict};
use crate::refs::Ref;
use crate::repository::Repository;

#[derive(Parser)]
#[command(about, long_about = None)]
struct Args {
    #[command(subcommand)]
    command: Option<Commands>
}

#[derive(Subcommand)]
#[derive(Debug)]
enum Commands {
    Init {
        #[arg(help="The path where the repo will be initiated")]
        path: String
    },
    // TODO: cat file could take a path to help the repo search for testing purposes it would help
    CatFile {
        #[arg(help="The type of object: commit, blob etc.")]
        object_type: String,
        #[arg(help="The name of the object e.g. the sha hash (that is split to create dir structure")]
        object_name: String
    },
    HashObject {
        #[arg(help="The type of object: commit, blob etc.")]
        object_type: String,
        #[arg(help="The path of the object to hash")]
        object_path: String,
        #[arg(help="If true, actually write the object")]
        write: bool
    },
    Checkout {
        #[arg(help="The commit or tree to checkout")]
        commit: String,
        #[arg(help="The EMPTY directory to checkout to")]
        path: String
    },
    CheckIgnore {
        #[arg(help="Paths to check. Return paths that will be ignored")]
        paths: Vec<String>
    },
    Tag {
        #[arg(short = 'a', help="If set we create a tag object")]
        store_true: bool,
        #[arg(help="The new tags name")]
        name: String,
        #[arg(help="The object the new tag will point to", default_value = "HEAD")]
        object: String
    },
    LsFiles,
    ShowRef,
    Status
}

pub struct Cli {
    args: Args
}

impl Cli {
    pub fn new() -> Self {
        Cli {
            args: Args::parse()
        }
    }

    pub fn run(&self) {
        match &self.args.command {
            None => panic!("Couldnt process command"),
            Some(cmd) => &self.process_command(cmd)
        };
    }

    fn process_command(&self, command: &Commands) {
        let result = match command {
            Commands::Init { path } => self.process_init(path),
            Commands::CatFile { object_type, object_name } => self.process_cat_file(object_type, object_name),
            Commands::HashObject { object_type, object_path, write } => self.process_hash_object(object_type, object_path, write),
            Commands::Checkout { commit, path } => self.process_checkout(commit, path),
            Commands::ShowRef => self.process_show_ref(),
            Commands::CheckIgnore { paths } => self.process_check_ignore(paths),
            Commands::Tag { store_true, name, object } => self.process_tag(store_true, name, object),
            Commands::LsFiles => self.process_ls_files(),
            Commands::Status => self.process_status().map_err(|_| "failed status")
        };

        match result {
            Err(err) => println!("ERROR in command run: {}", err),
            _ => {}
        }
    }

    fn process_init(&self, path: &String) -> Result<(), &'static str> {
        println!("Running init on: {}", path.clone());
        Repository::create(path.clone()).and_then(|_| Ok(()))
    }

    fn process_cat_file(&self, object_type: &String, object_name: &String) -> Result<(), &'static str> {
        println!("Running cat file on {} {}", object_type.clone(), object_name.clone());

        let repo = Repository::find(String::from("."), true).unwrap();
        let obj_name = object_find(&repo, &object_name, &object_type, true);
        let obj = object_read(&repo, obj_name).unwrap();

        println!("{:?}", obj);
        Ok(())
    }

    fn process_hash_object(&self, object_type: &String, object_path: &String, write: &bool) -> Result<(), &'static str> {
        todo!("Hash object cli not implemented yet, called for {} {} {}", object_type, object_path, write);
    }

    pub fn process_checkout(&self, commit: &String, path: &String) -> Result<(), &'static str> {
        let path_obj = Path::new(path);

        if path_obj.exists() {
            if !path_obj.is_dir() {
                return Err("Not a directory");
            }

            if !path_obj.read_dir().unwrap().next().is_none() {
                return Err("Directory not empty");
            }
        } else {
            create_dir_all(path_obj).unwrap();
        }

        let repo = Repository::find(String::from("."), true).unwrap();

        // TODO: technically this should support directly checking out a tree too but...
        let commit_obj_name = object_find(&repo, &commit, &"commit".to_string(), true);
        let commit_obj = match object_read(&repo, commit_obj_name).unwrap() {
            Commit(obj) => obj,
            _ => {
                return Err("Object not a commit");
            }
        };

        let tree_obj = commit_obj.get_and_read_tree(&repo).unwrap();
        tree_obj.checkout(&repo, canonicalize(path_obj).unwrap().as_path());

        Ok(())
    }

    fn process_show_ref(&self) -> Result<(), &'static str> {
        let repo = Repository::find(String::from("."), true).unwrap();
        let refs = Ref::all_refs(&repo);

        for item in refs {
            let resolution = item.fully_resolve(&repo);
            println!("Ref: {}, Target: {:?}, resolves to {:?}", item.name, item.target, resolution);
        }

        Ok(())
    }

    fn process_check_ignore(&self, paths: &Vec<String>) -> Result<(), &'static str> {
        let repo = Repository::find(String::from("."), true).unwrap();
        let ignore = Ignore::read(&repo);

        for path in paths {
            if let Some(result) = ignore.check_ignore(path.clone()) {
                if result {
                    println!("{}", path);
                }
            }
        }

        Ok(())
    }

    fn process_tag(&self, store_true: &bool, name: &String, object: &String) -> Result<(), &'static str> {
        let repo = Repository::find(String::from("."), true).unwrap();

        let tag = if *store_true {
            GitTag::new_object(name.clone(), object.clone(), &repo)
        } else {
            GitTag::new_lightweight(name.clone(), object.clone(), &repo)
        };

        object_write(GitObject::Tag(tag), Some(&repo)).and_then(|sha| {
            println!("Created tag with hash: {}", sha);
            Ok(())
        })
    }

    fn process_ls_files(&self) -> Result<(), &'static str> {
        let repo = Repository::find(String::from("."), true)?;
        // TODO: would be nice to use '?' op here but struggling to convert Err String to Err &'static str
        //       do I need a pass over whole codebase to unify errors to be String instead? could be much nicer
        //       that way I can use e.into() or format!("{}", e) to convert err to String in many cases much more nicely
        let index = Index::read(&repo).unwrap();

        for entry in index.entries {
            println!("{:?}", entry);
        }

        Ok(())
    }

    fn process_status(&self) -> Result<(), String> {
        let repo = Repository::find(String::from("."), true)?;

        if let Some(branch) = branch_get_active(&repo) {
            println!("Active branch: {}", branch);
        } else {
            println!("HEAD detached at {}", object_find(&repo, &"HEAD".to_string(), &"".to_string(), true));
        }

        self.print_status_head_index(&repo)?;

        Ok(())
    }

    fn print_status_head_index(&self, repo: &Repository) -> Result<(), String> {
        let index = Index::read(&repo)?;
        let mut head = tree_to_dict(&repo, &"HEAD".to_string(), None);

        for entry in index.entries {
            if head.contains_key(&entry.name) {
                if head.get(&entry.name).unwrap() != &entry.sha {
                    println!("\tmodified: {}", entry.name);
                }

                head.remove(&entry.name);
            } else {
                println!("\tadded: {}", entry.name);
            }
        }

        for entry in head.keys() {
            println!("\tdeleted: {}", entry);
        }

        Ok(())

    }

}