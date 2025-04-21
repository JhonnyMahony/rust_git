#[allow(unused_imports)]
use std::env;
#[allow(unused_imports)]
use std::fs;

use clap::Parser;
use cli::Args;
use cli::Commands;

mod cli;
mod commands;

fn main() {
    // You can use print statements as follows for debugging, they'll be visible when running tests.
    eprintln!("Logs from your program will appear here!");

    let args = Args::parse();
    match args.command {
        Commands::Init(arguments) => arguments.handle(),
        Commands::CatFile(arguments) => arguments.handle(),
        Commands::HashObject(arguments) => arguments.handle(),
        Commands::LsTree(arguments) => arguments.handle(),
        Commands::WriteTree(arguments) => arguments.handle(),
        Commands::CommitTree(arguments) => arguments.handle(),
    }
}
