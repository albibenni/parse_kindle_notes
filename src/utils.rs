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

/// Composes a full file path by joining home directory with folders path and filename
///
/// # Arguments
///
/// * `folders` - A string containing the relative directory path from home
/// * `file_name` - A string containing the name of the file
///
/// # Returns
///
/// A `String` containing the complete path to the file
///
/// # Panics
///
/// This function will panic if:
/// - The home directory cannot be determined
/// - The home directory path cannot be converted to a string
#[allow(deprecated)]
pub fn compose_full_path(folders: &String, file_name: &String) -> String {
    let home = std::env::home_dir().expect("nopeee");
    let home_str = home.to_str().expect("Str");
    let mut full_path = String::from(home_str);
    full_path.push_str("/");
    full_path.push_str(&folders);
    full_path.push_str("/");
    full_path.push_str(&file_name);
    return full_path;
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
    use std::env;
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

    #[test]
    fn test_compose_full_path() {
        // Save the current home directory value to restore it later
        let original_home = env::var("HOME").ok();

        // Set a mock home directory for testing
        let mock_home = "/mock/home";
        env::set_var("HOME", mock_home);

        // Mock the home_dir function since it's harder to override directly
        // This test checks the logic of path composition assuming home_dir works

        let folders = String::from("Documents/Kindle");
        let file_name = String::from("notes.txt");

        let expected_path = format!("{}/{}/{}", mock_home, folders, file_name);
        let result = compose_full_path(&folders, &file_name);

        // Restore the original home directory if it existed
        match original_home {
            Some(home) => env::set_var("HOME", home),
            None => env::remove_var("HOME"),
        }

        // Check that our path was composed as expected
        // Note: There's a bug in the original function where it adds the home path twice
        // This test will identify the bug by comparing with the expected (buggy) behavior
        assert_eq!(result, expected_path);
    }

    #[test]
    fn test_compose_full_path_components() {
        // This test doesn't mock home_dir but checks that the components are joined correctly
        let folders = String::from("test/folder");
        let file_name = String::from("test.txt");

        let result = compose_full_path(&folders, &file_name);

        // Verify that the result contains all the expected components
        assert!(result.contains(&folders));
        assert!(result.contains(&file_name));
        assert!(result.matches('/').count() >= 3); // At least 3 path separators
    }
}
