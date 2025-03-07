use crate::handle_kindle_notes;

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
pub fn file_check(s: &str, extension_supported: &str) -> bool {
    if !handle_kindle_notes::check_file_name(s, extension_supported) {
        println!(
            "Provide a valid filename that ends with: {}",
            extension_supported
        );
        return false;
    }
    return true;
}

/// Displays usage instructions for the Kindle notes parser application
///
/// # Description
///
/// Prints the different ways to use the application to stdout:
/// - With one argument: Specify just the file name
/// - With two arguments: Specify both folder and file name
///
/// # Examples
///
/// ```
/// use kindle_notes_parser::utils::help;
/// help(); // Prints usage information to stdout
/// ```
pub fn help() {
    println!("Usage: 1 Argument: kindle-notes-parser <file>");
    println!("file: The file containing the Kindle notes, ending with .txt");
    println!("Usage: 2 Arguments: kindle-notes-parser <folder> <file>");
    println!("folder: The folder where the file is located");
    println!("file: The file containing the Kindle notes, ending with .txt");
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_file_check_valid_extension() {
        // Temporarily capture stdout to avoid printing during tests
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
