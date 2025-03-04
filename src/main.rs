#[path = "./handle_kindle_notes.rs"]
mod handle_kindle_notes;

fn main() {
    println!("Hello, world!");

    let mut args = std::env::args();
    // .nth(1)
    // .expect("provide a file name as argument");

    let extension_supported = ".txt";
    match args.len() {
        1 => {
            println!("provide a file name as argument");
            return;
        }
        2 => {
            let s = args.nth(1).expect("provide a file name as argument");
            handle_kindle_notes::check_file_name(&s, extension_supported);
            handle_kindle_notes::parse_kindle_notes(&s);
        }
        3 => {
            //let cmd = args.nth(1).expect("provide a cmd")
            let res: Vec<String> = args.into_iter().collect();
            println!("{:?}", res);
            //handle_kindle_notes::parse_kindle_notes(path);
        }
        _ => {
            println!("provide a file name as argument");
            return;
        }
    }

    // let mut path: String = String::from("~/").to_owned();
    // path.push_str(&args.nth(1).unwrap());
    // handle_kindle_notes::parse_kindle_notes(path);
}
