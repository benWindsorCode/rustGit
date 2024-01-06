use clap::{Parser, Subcommand};
use rust_git::repository::Repository;

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
    Init
}

fn main() {
    let cli = Args::parse();

    println!("{:?}", cli.command);

    Repository::create(String::from("C:\\Users\\benja\\Documents\\code\\rustGit\\test")).unwrap();
}
