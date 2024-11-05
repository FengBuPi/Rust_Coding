fn main() {
    let cargo_toml_content = std::fs::read_to_string("text.txt").unwrap();
    println!("'text.txt':\n{}", cargo_toml_content);
}
