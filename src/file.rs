use std::fs;

pub fn get_size_bytes(filename: &String) -> usize {
    let content = fs::read_to_string(filename).expect("file not found");
    content.len()
}
