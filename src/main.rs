use std::{
    env,
    fs::{read_to_string, File},
    io::Write,
};

use toml_edit::Document;

fn read_to_document(path: &str) -> Document {
    read_to_string(path)
        .expect("Failed to read file")
        .parse()
        .expect("Failed to parse content")
}

fn sort_block(document: &mut Document, key: &str) {
    if let Some(block) = document.get_mut(key) {
        block
            .as_table_mut()
            .expect("failed to cast to table")
            .sort_values();
    }
}

fn save_to_file(document: Document, path: &str) {
    let mut credentials_config = File::create(path).expect("Failed to create file");

    credentials_config
        .write_all(document.to_string().as_bytes())
        .expect("Failed to write file");
}

pub fn get_path() -> String {
    env::args().nth(1).expect("Path isn't set")
}

fn main() {
    let path = get_path();

    let mut document = read_to_document(path.as_str());

    sort_block(&mut document, "dependencies");
    sort_block(&mut document, "dev-dependencies");

    save_to_file(document, path.as_str());
}
