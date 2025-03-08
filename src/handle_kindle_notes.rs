pub fn parse_kindle_notes(path: &str, book_title: &str) {
    println!("{}", path);
    let file: String = std::fs::read_to_string(path).expect("No file found.");

    let repl = &file.replace("\u{feef}", "");
    let lines: &Vec<&str> = &repl.split("\n").collect();
    // if lines.get(0).expect("No note found").contains("\u{feef}") {
    //     let replac = lines[0].replace("\u{feef}", "");
    // }
    let mut parsed_file: Vec<&str> = Vec::new();
    // book_title ckeck to check what to parse
    let mut is_book_title = false;
    for idx in 0..lines.len() {
        let line: &str = lines.get(idx).expect("Wrong index");
        // book_title ckeck
        if line.to_lowercase().starts_with(&book_title.to_lowercase()) {
            println!("whatt{}", line);
            is_book_title = true;
            continue;
        }
        // parse only book title paragraphs
        if !is_book_title {
            is_book_title = false;
            continue;
        }
        if line == "==========" {
            parsed_file.push(" ");
            parsed_file.push("---");
            is_book_title = false;
            continue;
        }
        if line.trim() == ""
            || line.starts_with("- Your Highlight at location")
            || line.starts_with("- Your Highlight on page")
            || line.starts_with(book_title)
        {
            continue;
        }
        parsed_file.push(line);
    }

    for l in parsed_file {
        println!("LOL: {:?}", l);
    }
}

/// Checks if a given file name ends with .txt extension
///
/// # Arguments
///
/// * `file_name` - The name of the file to check
///
/// # Returns
///
/// `true` if the file name ends with ".txt", otherwise `false`
pub fn check_file_name(file_name: &str, extension_supported: &str) -> bool {
    let ends_with: bool = file_name.ends_with(extension_supported);
    return ends_with && file_name.len() > extension_supported.len();
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_check_file_name() {
        let extension_supported = ".txt";
        assert!(check_file_name("notes.txt", extension_supported));
        assert!(check_file_name("My Kindle Notes.txt", extension_supported));
        assert!(!check_file_name("notes.pdf", extension_supported));
        assert!(!check_file_name("notes.txt.pdf", extension_supported));
        assert!(!check_file_name("notes", extension_supported));
        assert!(!check_file_name(".txt", extension_supported));
    }
}
