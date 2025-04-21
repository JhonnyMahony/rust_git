use crate::commands::{cat_file, commit_tree, hash_object, init, ls_tree, write_tree};
use clap::{Parser, Subcommand};

#[derive(Parser, Debug)]
#[command(version, about)]
pub struct Args {
    #[clap(subcommand)]
    /// These are common Git commands used in various situations:
    pub command: Commands,
}

#[derive(Subcommand, Debug)]
pub enum Commands {
    /// Create an empty Git repository or reinitialize an existing one
    Init(init::Arguments),
    /// Provide contents or details of repository objects
    CatFile(cat_file::Arguments),
    /// Compute object ID and optionally create an object from a file
    HashObject(hash_object::Arguments),
    /// List the contents of a tree object
    LsTree(ls_tree::Arguments),
    /// Create a tree object from the current index
    WriteTree(write_tree::Arguments),
    ///
    CommitTree(commit_tree::Arguments),
}
