use std::{
    fs,
    io::{Read, Write},
    path::PathBuf,
};

use directories::ProjectDirs;

const QUALIFIER: &str = "test";
const ORGANIZATION: &str = "test";
const APPLICATION: &str = "flappy_bevy";
const KV_DIR: &str = "kv";

pub fn save(key: &str, val: &str) {
    if let Some(proj_dirs) = ProjectDirs::from(QUALIFIER, ORGANIZATION, APPLICATION) {
        let path = proj_dirs.data_dir();
        let mut file_path = PathBuf::from(path);
        file_path.push(KV_DIR);

        if let Err(e) = fs::create_dir_all(&file_path) {
            println!("dir create fail: {}", e);
            return;
        }
        file_path.push(key);
        let mut file = match fs::File::create(&file_path) {
            Ok(file) => file,
            Err(e) => {
                println!("file create fail err: {}", e);
                return;
            }
        };

        if let Err(e) = file.write_all(val.as_bytes()) {
            println!("file wrtie fail err: {}", e);
            return;
        }
    }
}

pub fn load(key: &str) -> String {
    if let Some(proj_dirs) = ProjectDirs::from(QUALIFIER, ORGANIZATION, APPLICATION) {
        let path = proj_dirs.data_dir();
        let mut file_path = PathBuf::from(path);
        file_path.push(KV_DIR);
        file_path.push(key);
        let mut file = match fs::File::open(&file_path) {
            Ok(file) => file,
            Err(e) => {
                println!("file open fail err: {}", e);
                return String::new();
            }
        };
        let mut contents = String::new();
        if let Err(e) = file.read_to_string(&mut contents) {
            println!("file read faile err: {}", e);
            return String::new();
        }
        contents
    } else {
        String::new()
    }
}
