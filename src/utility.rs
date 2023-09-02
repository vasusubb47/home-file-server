use rand::{distributions::Alphanumeric, Rng};
use std::{env, fmt::Write, path::PathBuf};

pub mod api;
pub mod jwt_token;

pub fn u8_to_hex_str(bytes: &[u8]) -> String {
    let mut hex_str = String::with_capacity(bytes.len() * 2);
    for &b in bytes {
        write!(&mut hex_str, "{:02x}", b).unwrap();
    }
    hex_str
}

pub fn genarate_salt(salt_len: usize) -> String {
    rand::thread_rng()
        .sample_iter(&Alphanumeric)
        .take(salt_len)
        .map(char::from)
        .collect()
}

pub fn get_file_type(file_path: &str) -> String {
    let path = PathBuf::from(file_path);
    let ext = path.extension().unwrap();
    ext.to_str().unwrap().to_owned()
}

pub fn get_current_working_dir() -> std::io::Result<PathBuf> {
    env::current_dir()
}
