use clap::Parser;
use flate2::{write::ZlibEncoder, Compression};
use sha1::{Digest, Sha1};
use std::{
    fs,
    io::{BufReader, Read, Write},
};

#[derive(Parser, Debug)]
pub struct Arguments {
    file: String,
}

impl Arguments {
    pub fn handle(&self) {
        let data = fs::read(&self.file).unwrap();
        let header = format!("blob {}\0", data.len());

        let mut hasher = Sha1::new();
        hasher.update(&header);
        hasher.update(&data);
        let hash = format!("{:x}", hasher.finalize());

        let mut encoder = ZlibEncoder::new(Vec::new(), Compression::default());
        encoder.write_all(header.as_bytes()).unwrap();
        encoder.write_all(&data).unwrap();
        let compressed_bytes = encoder.finish().unwrap();

        let (group, name) = hash.split_at(2);
        fs::create_dir(format!(".git/objects/{}", group)).unwrap();
        fs::write(format!(".git/objects/{}/{}", group, name), compressed_bytes).unwrap();
    }
}
