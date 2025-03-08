pub fn parse_kindle_notes(path: &str) {
    println!("{}", path);
    let file: String = std::fs::read_to_string(path).expect("No file found.");

    let lines: Vec<&str> = file.split("\n").collect();
    //println!("{}", &file);
    let mut v: Vec<&str> = Vec::new();
    let book_title = "tbd";
    for idx in 0..lines.len() {
        let line: &str = lines.get(idx).expect("Wrong index");
        if line.to_lowercase().starts_with(&book_title.to_lowercase()) {}
        if line == "==========" {
            v.push(" ");
            v.push("---");
        }
        if line.trim() == ""
            || line.starts_with("- Your Highlight at location")
            || line.starts_with("- Your Highlight on page")
            || line.starts_with(book_title)
        {
            continue;
        }
        //notes.push(line);
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
