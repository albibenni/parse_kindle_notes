#[path = "./handle_kindle_notes.rs"]
mod handle_kindle_notes;
fn main() {
    println!("Hello, world!");
    let path = String::from("test");
    handle_kindle_notes::parse_kindle_notes(path);
}
