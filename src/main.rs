use utils::compose_full_path;

#[path = "./handle_kindle_notes.rs"]
mod handle_kindle_notes;

#[path = "./utils.rs"]
mod utils;

fn main() {
    println!("Hello, Notes!");

    let args = std::env::args();

    let extension_supported = ".txt";
    match args.len() {
        3 => {
            let arguments: Vec<String> = args.map(|arg| arg).collect();
            //let book_title = "effective typescript";
            let book_title: &String = arguments
                .get(2)
                .expect("Provide a book title as third argument, eg: effective'\' typescript");
            let file_name: &String = arguments
                .get(1)
                .expect("Provide a file name as first argument");
            if !utils::file_check(&file_name, &extension_supported) {
                return;
            }

            handle_kindle_notes::parse_kindle_notes(&file_name, &book_title);
        }
        4 => {
            let arguments: Vec<String> = args.map(|arg| arg).collect();
            let book_title: &String = arguments
                .get(3)
                .expect("Provide a book title as third argument, eg: effective'\' typescript");
            let file_name: &String = arguments
                .get(2)
                .expect("Provide a file name as second argument");
            if !utils::file_check(&file_name, &extension_supported) {
                return;
            }
            let folders: &String = arguments
                .get(1)
                .expect("Provide a folder path as first argument");

            let full_path = compose_full_path(&folders, &file_name);
            handle_kindle_notes::parse_kindle_notes(&full_path, &book_title);
        }
        _ => {
            utils::help();
            return;
        }
    }
}
