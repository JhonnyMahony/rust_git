use std::{fs, io::Read, str::from_utf8};

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
                    let mut decoded_data = Vec::new();
                    decode.read_to_end(&mut decoded_data).unwrap();
                    let header = decoded_data.iter().position(|&byte| byte == b'\0').unwrap();
                    let s = from_utf8(&decoded_data[header..]).unwrap();
                    print!("{}", s)
                }
                Err(_) => {
                    println!("Failed read data")
                }
            }
        }
    }
}
