use clap::{Parser, Subcommand};
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
    Add,
    Commit,
    Init {
        #[arg(help="The path where the repo will be initiated")]
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
        match command {
            Commands::Add => todo!("Add not implemented"),
            Commands::Commit => todo!("Commit not implemented"),
            Commands::Init { path } => self.process_init(path)
        }
    }

    fn process_init(&self, path: &String) {
        println!("Running init on: {}", path.clone());
        Repository::create(path.clone()).unwrap();
    }
}