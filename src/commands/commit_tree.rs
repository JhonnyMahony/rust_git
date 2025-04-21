use std::{fs, io::Write};

use clap::{Args, Parser};
use flate2::{write::ZlibEncoder, Compression};
use sha1::{Digest, Sha1};

#[derive(Parser, Debug)]
pub struct Arguments {
    #[command(flatten)]
    flags: Flags,
    tree: String,
}

#[derive(Args, Debug)]
pub struct Flags {
    #[arg(short)]
    parent_tree: Option<String>,
    #[arg(short)]
    message: String,
}

impl Arguments {
    pub fn handle(&self) {
        let mut commit = String::new();
        commit.push_str(&format!("tree {}\n", self.tree));
        if let Some(parent) = &self.flags.parent_tree {
            commit.push_str(&format!("parent {}\n", parent));
        }
        commit.push_str(&get_author());
        commit.push_str(&get_commiter());
        commit.push_str(&format!("\n{}", self.flags.message));
        let header = format!("commit {}\0", commit.len());
        let mut hasher = Sha1::new();
        hasher.update(&header);
        hasher.update(&commit);
        let hash = format!("{:x}", hasher.finalize());
        let mut compressor = ZlibEncoder::new(Vec::new(), Compression::default());
        compressor.write_all(header.as_bytes()).unwrap();
        compressor.write_all(commit.as_bytes()).unwrap();
        let compressed_bytes = compressor.finish().unwrap();
        let (group, name) = hash.split_at(2);

        let _ = fs::create_dir(format!(".git/objects/{}", group));
        fs::write(format!(".git/objects/{}/{}", group, name), compressed_bytes).unwrap();
        println!("{}", commit);
        println!("{}", hash);
    }
}

fn get_author() -> String {
    String::from("author JhonnyMahony <rallidars@gmail.com> 1745226330 +0300\n")
}

fn get_commiter() -> String {
    String::from("committer JhonnyMahony <rallidars@gmail.com> 1745226330 +0300\n")
}
