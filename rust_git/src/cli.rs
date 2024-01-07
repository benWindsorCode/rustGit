use clap::{Parser, Subcommand};
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
    CatFile {
        #[arg(help="The type of object: commit, blob etc.")]
        object_type: String,
        #[arg(help="The name of the object e.g. the sha hash (that is split to create dir structure")]
        object_name: String
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
        match command {
            Commands::Init { path } => self.process_init(path),
            Commands::CatFile { object_type, object_name } => self.process_cat_file(object_type, object_name)
        }
    }

    fn process_init(&self, path: &String) {
        println!("Running init on: {}", path.clone());
        Repository::create(path.clone()).unwrap();
    }

    fn process_cat_file(&self, object_type: &String, object_name: &String) {
        println!("Running cat file on {} {}", object_type.clone(), object_name.clone());

        let repo = Repository::find(String::from("."), true).unwrap();
        let obj_name = object_find(&repo, &object_name, &object_type, true);
        let obj = object_read(&repo, obj_name).unwrap();

        println!("{:?}", obj);
    }
}