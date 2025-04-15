use std::{fs, io::Write};

use clap::Parser;
use flate2::{write::ZlibEncoder, Compression};
use sha1::{Digest, Sha1};

#[derive(Parser, Debug)]
pub struct Arguments {}

impl Arguments {
    pub fn handle(&self) {
        let index_path = ".";
        println!("{:?}", write_tree(index_path));
    }
}

#[derive(Debug, Clone)]
pub struct Entry {
    pub mode: String,
    pub file_name: String,
    pub hash: String,
}
impl Entry {
    fn new(mode: String, file_name: String, hash: String) -> Self {
        Self {
            mode,
            file_name,
            hash,
        }
    }
}

fn write_tree(path: &str) -> Option<String> {
    let mut entries: Vec<Entry> = Vec::new();
    let dir = fs::read_dir(path).unwrap();
    for p in dir {
        let path = p.unwrap().path();
        let file_name = path.file_name().unwrap().to_str().unwrap().to_string();
        if file_name.starts_with(".git") {
            continue;
        }

        if path.is_dir() {
            if let Some(hash) = write_tree(path.to_str().unwrap()) {
                let entry = Entry::new("40000".to_string(), file_name.clone(), hash);
                entries.push(entry.clone());
                println!("hash of dir: {:?}", entry)
            }
        }
        if path.is_file() {
            let data = fs::read(path).unwrap();
            let header = format!("blob {}\0", data.len());
            let mut hasher = Sha1::new();
            hasher.update(&header);
            hasher.update(&data);
            let hash = format!("{:x}", hasher.finalize());
            let entry = Entry::new("100644".to_string(), file_name.clone(), hash.clone());
            let mut encoder = ZlibEncoder::new(Vec::new(), Compression::default());
            encoder.write_all(header.as_bytes()).unwrap();
            encoder.write_all(&data).unwrap();
            let data = encoder.finish().unwrap();
            let (group, name) = hash.split_at(2);
            let _ = fs::create_dir(format!(".git/objects/{}", group));
            fs::write(format!(".git/objects/{}/{}", group, name), data).unwrap();
            entries.push(entry);
        }
    }

    if entries.is_empty() {
        return None;
    }

    entries.sort_by(|a, b| a.file_name.cmp(&b.file_name));

    let mut data: Vec<u8> = Vec::new();
    for entry in entries {
        let mode_and_name = format!("{} {}\0", entry.mode, entry.file_name);
        data.extend_from_slice(mode_and_name.as_bytes());
        let converted_hash: Vec<u8> = (0..entry.hash.len())
            .step_by(2)
            .map(|i| u8::from_str_radix(&entry.hash[i..i + 2], 16).unwrap())
            .collect();
        data.extend_from_slice(&converted_hash);
    }
    let header = format!("tree {}\0", data.len());
    let mut hasher = Sha1::new();
    let mut tree_object = Vec::new();
    tree_object.extend_from_slice(header.as_bytes());
    tree_object.extend_from_slice(&data);
    hasher.update(tree_object);
    let hash = format!("{:x}", hasher.finalize());

    let mut encoder = ZlibEncoder::new(Vec::new(), Compression::default());
    encoder.write_all(header.as_bytes()).unwrap();
    encoder.write_all(&data).unwrap();
    let data = encoder.finish().unwrap();
    let (group, name) = hash.split_at(2);
    let _ = fs::create_dir(format!(".git/objects/{}", group));
    fs::write(format!(".git/objects/{}/{}", group, name), data).unwrap();

    Some(hash)
}

// 3b  5e  d9  e6
