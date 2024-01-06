use clap::{Parser, Subcommand};
use rust_git::cli::Cli;
use rust_git::repository::Repository;


fn main() {
    let cli = Cli::new();
    cli.run();
    // Repository::create(String::from("C:\\Users\\benja\\Documents\\code\\my_git_test")).unwrap();
}
