use rand::{distributions::Alphanumeric, Rng};
use std::fmt::Write;

pub mod api;

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
