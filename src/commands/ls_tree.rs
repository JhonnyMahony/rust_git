use std::{fs, io::Read, str::from_utf8};

use clap::{ArgAction, Args, Parser};
use flate2::read::ZlibDecoder;

#[derive(Debug, Parser)]
pub struct Arguments {
    object: String,
    #[command(flatten)]
    flags: Flags,
}

#[derive(Args, Debug)]
struct Flags {
    #[arg(long, action = ArgAction::SetTrue)]
    name_only: bool,
}

#[derive(Debug)]
pub struct TreeObject {
    mode: String,
    name: String,
    obj_type: String,
    hash: String,
}
impl TreeObject {
    pub fn to_string_name(&self) -> String {
        self.name.clone()
    }
    pub fn to_string(&self) -> String {
        format!(
            "{} {} {}\t{}",
            self.mode, self.obj_type, self.hash, self.name
        )
    }
}

impl Arguments {
    pub fn handle(&self) {
        let (group, name) = self.object.split_at(2);
        let path = format!(".git/objects/{}/{}", group, name);

        let file = fs::read(path).unwrap();
        let mut decoder = ZlibDecoder::new(&*file);
        let mut decoded_data = Vec::new();
        decoder.read_to_end(&mut decoded_data).unwrap();
        let header = decoded_data.iter().position(|&byte| byte == b'\0').unwrap() + 1;
        let data_without_header = &decoded_data[header..];
        let trees = parse_tree_object(data_without_header);
        if self.flags.name_only {
            for tree in trees {
                println!("{}", tree.to_string_name())
            }
        } else {
            for tree in trees {
                println!("{}", tree.to_string())
            }
        }
    }
}

fn parse_tree_object(data: &[u8]) -> Vec<TreeObject> {
    let mut tree_obgects = Vec::new();
    let mut data = data;
    while data.len() > 0 {
        let mode_and_name_end = data.iter().position(|&byte| byte == b'\0').unwrap();

        let splited: Vec<&str> = from_utf8(&data[..mode_and_name_end])
            .unwrap()
            .split_whitespace()
            .collect();
        let (mode, name) = (splited[0], splited[1]);
        let normalized_mode = if mode.len() == 5 {
            format!("0{}", mode)
        } else {
            mode.to_string()
        };
        let obj_type = match mode {
            "100644" | "100755" => "blob",
            "40000" => "tree",
            _ => mode,
        };
        let new_data = &data[mode_and_name_end + 1..];
        let mut hash = String::new();

        for b in &new_data[..20] {
            hash.push_str(&format!("{:02x}", b))
        }
        let tree = TreeObject {
            mode: normalized_mode,
            name: name.to_string(),
            obj_type: obj_type.to_string(),
            hash,
        };
        tree_obgects.push(tree);
        data = &new_data[20..]
    }

    tree_obgects
}
