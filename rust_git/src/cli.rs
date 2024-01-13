use std::fs::{canonicalize, create_dir_all};
use std::path::Path;
use clap::{Parser, Subcommand};
use crate::git_object::GitObject::Commit;
use crate::object_utils::{object_find, object_read};
use crate::repository::Repository;

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
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
    }
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
            Commands::Checkout { commit, path } => self.process_checkout(commit, path)
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
}