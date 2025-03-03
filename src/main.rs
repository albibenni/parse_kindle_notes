#[path = "./handle_kindle_notes.rs"]
mod handle_kindle_notes;
fn main() {
    println!("Hello, world!");

    let arg: String = std::env::args()
        .nth(1)
        .expect("provide a file name as argument");
    let mut path: String = String::from("~/").to_owned();
    path.push_str(&arg);
    handle_kindle_notes::parse_kindle_notes(path);
}
