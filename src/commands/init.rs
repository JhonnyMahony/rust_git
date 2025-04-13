use clap::Parser;
use std::fs;

#[derive(Parser, Debug)]
pub struct Arguments {}

impl Arguments {
    pub fn handle(&self) {
        fs::create_dir(".git").unwrap();
        fs::create_dir(".git/objects").unwrap();
        fs::create_dir(".git/refs").unwrap();
        fs::write(".git/HEAD", "ref: refs/heads/main\n").unwrap();
        println!("Initialized git directory")
    }
}
