use std::{
    fs,
    io::{Read, Write},
};

use clap::{Args, Parser};
use flate2::read::ZlibDecoder;

#[derive(Parser, Debug)]
pub struct Arguments {
    #[command(flatten)]
    flags: Flags,
    object: String,
}

#[derive(Args, Debug)]
#[group(required = true)]
struct Flags {
    #[arg(short)]
    /// check if <object> exists
    pretty_print: bool,
}

impl Arguments {
    pub fn handle(&self) {
        if self.flags.pretty_print {
            let (group, name) = self.object.split_at(2);
            let path = format!(".git/objects/{}/{}", group, name);
            match fs::read(path) {
                Ok(file_data) => {
                    let mut decode = ZlibDecoder::new(&*file_data);
                    let mut decoded_data = String::new();
                    decode.read_to_string(&mut decoded_data).unwrap();
                    print!("{}", &decoded_data[8..])
                }
                Err(_) => {
                    println!("Failed read data")
                }
            }
        }
    }
}
