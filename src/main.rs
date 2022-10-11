use std::{env, fs::read_to_string};

use toml_edit::{Document, Table};

fn read_to_document(path: &str) -> Document {
    read_to_string(path)
        .expect("Failed to read file")
        .parse()
        .expect("Failed to parse content")
}

fn get_table<'a>(document: &'a mut Table, key: &str) -> Option<&'a mut Table> {
    document.get_mut(key)?.as_table_mut()
}

fn save_to_file(document: Document, path: &str) {
    std::fs::write(path, document.to_string().as_bytes()).expect("Failed to save file")
}

pub fn get_path() -> String {
    env::args().nth(1).expect("Path isn't set")
}

fn sort_dependencies(document: &mut Table) {
    if let Some(table) = get_table(document, "dependencies") {
        table.sort_values()
    }

    if let Some(table) = get_table(document, "dev-dependencies") {
        table.sort_values()
    }
}

fn sort(table: &mut Table) {
    sort_dependencies(table);

    if let Some(table) = get_table(table, "workspace") {
        sort_dependencies(table);
    }
}

fn main() {
    let path = get_path();

    let mut document = read_to_document(path.as_str());

    sort(document.as_table_mut());

    save_to_file(document, path.as_str());
}

#[cfg(test)]
mod tests {
    use super::*;

    const CARGO_TOML_WORKSPACE: &str = r#"
    [workspace]
    members = [
    "member1",
    "member2",
    ]

    [dependencies]
    b = "1"
    a = "1"
    z = "1"

    [dev-dependencies]
    b = "1"
    a = "1"
    z = "1"

    [workspace.dependencies]
    b = "1"
    a = "1"
    z = "1"

    [workspace.dev-dependencies]
    b = "1"
    a = "1"
    z = "1"
    "#;

    const CARGO_TOML_WORKSPACE_EXPECTED: &str = r#"
    [workspace]
    members = [
    "member1",
    "member2",
    ]

    [dependencies]
    a = "1"
    b = "1"
    z = "1"

    [dev-dependencies]
    a = "1"
    b = "1"
    z = "1"

    [workspace.dependencies]
    a = "1"
    b = "1"
    z = "1"

    [workspace.dev-dependencies]
    a = "1"
    b = "1"
    z = "1"
    "#;

    #[test]
    fn sort_toml() {
        let mut document: Document = CARGO_TOML_WORKSPACE.parse().unwrap();

        sort(document.as_table_mut());

        assert_eq!(&document.to_string(), CARGO_TOML_WORKSPACE_EXPECTED)
    }
}
