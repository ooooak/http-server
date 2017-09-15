use std::fs::File;
use std::path::PathBuf;
use std::io::prelude::*;

pub fn read(path: PathBuf) -> Option<Vec<u8>> {
    if path.is_file() {
        match File::open(&path) {
            Ok(mut file) => {
                let mut buffer = Vec::new();
                match file.read_to_end(&mut buffer) {
                    Ok(_) => Some(buffer),
                    Err(_) => None,
                }
            },
            Err(_) => None,
        }
    }else {
        None
    }  
}