pub fn parse_kindle_notes(path: &str) {
    println!("{}", path);
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
