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
            let file_name = args.nth(1).expect("provide a file name as argument");
            if !file_check(&file_name, &extension_supported) {
                return;
            }
            handle_kindle_notes::parse_kindle_notes(&file_name);
        }
        3 => {
            println!("3");
            let arguments: Vec<String> = args.map(|arg| arg).collect();
            println!("{:?}", arguments);
            // let folders = args
            //     .nth(1)
            //     .expect("provide a folder name as first argument");
            let file_name = arguments
                .get(2)
                .expect("Provide a file name as second argument");
            if !file_check(&file_name, &extension_supported) {
                return;
            }
            let folders: &String = arguments
                .get(1)
                .expect("Provide a folder path as first argument");
            let mut full_path = String::from("~/");
            full_path.push_str(&folders);
            full_path.push_str("/");
            full_path.push_str(&file_name);
            handle_kindle_notes::parse_kindle_notes(&full_path);
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

/// Checks if a file has the required extension
///
/// # Arguments
///
/// * `s` - A string slice representing the filename to check
/// * `extension_supported` - A string slice containing the required file extension
///
/// # Returns
///
/// `true` if the file has the required extension, `false` otherwise
fn file_check(s: &str, extension_supported: &str) -> bool {
    if !handle_kindle_notes::check_file_name(s, extension_supported) {
        println!(
            "Provide a valid filename that ends with: {}",
            extension_supported
        );
        return false;
    }
    return true;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_file_check_valid_extension() {
        // Temporarily capture stdout to avoid printing during tests
        //let result = std::io::sink();
        let _guard = stderrlog::StdErrLog::new().quiet(true).init().unwrap();

        assert!(file_check("test.txt", ".txt"));
        assert!(file_check("document.pdf", ".pdf"));
        assert!(file_check("notes with spaces.md", ".md"));
    }

    #[test]
    fn test_file_check_invalid_extension() {
        assert!(!file_check("test.doc", ".txt"));
        assert!(!file_check("document", ".pdf"));
        assert!(!file_check(".gitignore", ".md"));
    }
}
