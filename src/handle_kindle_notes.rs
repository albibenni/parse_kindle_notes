use std::fs;

const NOTE_PATH: &str = "NOTE_PATH";

pub fn parse_kindle_notes<E>(
    path: &str,
    book_title: &str,
) -> Result<Result<(), std::io::Error>, E> {
    dotenv::dotenv().ok();

    let file: String = std::fs::read_to_string(path).expect("No file found.");

    let repl: &String = &file.replace("\u{feef}", "");
    let lines: &Vec<&str> = &repl.split("\n").collect();
    let parsed_file = parse_file_by_book_title(&book_title, &lines);

    let parsed: String = parsed_file.join("\n");
    let new_path = new_write_path(&book_title);
    let res = fs::write(new_path, &parsed);
    return Ok(res);
}

/// Parses Kindle notes file content to extract highlights for a specific book title
///
/// # Arguments
///
/// * `book_title` - The title of the book to filter highlights for
/// * `lines` - A vector of string slices representing each line in the Kindle notes file
///
/// # Returns
///
/// A vector of string slices containing the parsed highlights from the specified book
///
/// # Details
///
/// The function processes the Kindle notes file as follows:
/// 1. Identifies sections that match the provided book title (case-insensitive)
/// 2. Collects highlight text until the section separator ("==========")
/// 3. Skips metadata lines like location markers and empty lines
/// 4. Adds formatting markers (spaces and "---") between highlights
fn parse_file_by_book_title<'a>(book_title: &str, lines: &Vec<&'a str>) -> Vec<&'a str> {
    let mut parsed_file: Vec<&str> = Vec::new();
    // book_title ckeck to check what to parse
    let mut is_book_title = false;
    for idx in 0..lines.len() {
        let line: &str = lines.get(idx).expect("Wrong index");
        // book_title ckeck
        if line.to_lowercase().starts_with(&book_title.to_lowercase()) {
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
    return parsed_file;
}

/// Generates a path for writing notes to a markdown file
///
/// # Arguments
///
/// * `book_title` - The title of the book to use for generating the filename
///
/// # Returns
///
/// A `String` containing the full path to the markdown file where notes will be saved
///
/// # Panics
///
/// This function will panic if:
/// - The environment variable specified by `NOTE_PATH` is not set
/// - The book title is empty
fn new_write_path(book_title: &str) -> String {
    let folder_base =
        std::env::var(NOTE_PATH).expect("Env var not found, provide a path to write the file");
    let mut book_title_chars = book_title.chars();
    let file_name = match book_title_chars.next() {
        None => panic!("Book title is empty!"),
        Some(c) => {
            folder_base
                + &c.to_uppercase().collect::<String>()
                + book_title_chars.as_str()
                + "/"
                + &c.to_uppercase().collect::<String>()
                + book_title_chars.as_str()
                + ".md"
        }
    };

    return file_name;
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
    use std::env;

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

    #[test]
    fn test_parse_file_by_book_title_empty_input() {
        let book_title = "Test Book";
        let lines: Vec<&str> = Vec::new();

        let result = parse_file_by_book_title(book_title, &lines);

        assert!(
            result.is_empty(),
            "Should return empty vector for empty input"
        );
    }

    #[test]
    fn test_parse_file_by_book_title_no_matching_book() {
        let book_title = "Test Book";
        let lines = vec![
            "Another Book",
            "This is a highlight",
            "==========",
            "More content",
        ];

        let result = parse_file_by_book_title(book_title, &lines);

        assert!(
            result.is_empty(),
            "Should return empty vector when book title doesn't match"
        );
    }

    #[test]
    fn test_parse_file_by_book_title_with_matching_book() {
        let book_title = "Test Book";
        let lines = vec![
            "Test Book",
            "This is a highlight",
            "==========",
            "Another Book",
            "This shouldn't be included",
            "==========",
        ];

        let result = parse_file_by_book_title(book_title, &lines);

        assert_eq!(result, vec!["This is a highlight", " ", "---"]);
    }

    #[test]
    fn test_parse_file_by_book_title_multiple_highlights() {
        let book_title = "Test Book";
        let lines = vec![
            "Test Book",
            "First highlight",
            "==========",
            "Test Book",
            "Second highlight",
            "==========",
            "Another Book",
            "Not included",
            "==========",
        ];

        let result = parse_file_by_book_title(book_title, &lines);

        assert_eq!(
            result,
            vec![
                "First highlight",
                " ",
                "---",
                "Second highlight",
                " ",
                "---"
            ]
        );
    }

    #[test]
    fn test_parse_file_by_book_title_skips_metadata() {
        let book_title = "Test Book";
        let lines = vec![
            "Test Book",
            "- Your Highlight at location 123",
            "- Your Highlight on page 45",
            "This is actual content",
            "",
            "More content",
            "==========",
        ];

        let result = parse_file_by_book_title(book_title, &lines);

        assert_eq!(
            result,
            vec!["This is actual content", "More content", " ", "---"]
        );
    }

    #[test]
    fn test_parse_file_by_book_title_case_insensitive() {
        let book_title = "Test Book";
        let lines = vec![
            "test book", // lowercase should still match
            "This is a highlight",
            "==========",
        ];

        let result = parse_file_by_book_title(book_title, &lines);

        assert_eq!(result, vec!["This is a highlight", " ", "---"]);
    }

    #[test]
    fn test_new_write_path_normal_title() {
        // Setup: Set the environment variable
        let temp_path = "/tmp/notes/";
        env::set_var(NOTE_PATH, temp_path);

        // Test with a normal book title
        let book_title = "the great gatsby";
        let result = new_write_path(book_title);

        // First character should be uppercase, and it should end with .md
        assert_eq!(result, format!("{}The great gatsby.md", temp_path));

        // Cleanup
        env::remove_var(NOTE_PATH);
    }

    #[test]
    fn test_new_write_path_single_word() {
        // Setup
        let temp_path = "/tmp/notes/";
        env::set_var(NOTE_PATH, temp_path);

        // Test with single word
        let book_title = "golang";
        let result = new_write_path(book_title);

        assert_eq!(result, format!("{}Golang.md", temp_path));

        // Cleanup
        env::remove_var(NOTE_PATH);
    }

    #[test]
    fn test_new_write_path_with_special_chars() {
        // Setup
        let temp_path = "/tmp/notes/";
        env::set_var(NOTE_PATH, temp_path);

        // Test with a title containing special characters
        let book_title = "object-oriented design";
        let result = new_write_path(book_title);

        assert_eq!(result, format!("{}Object-oriented design.md", temp_path));

        // Cleanup
        env::remove_var(NOTE_PATH);
    }

    #[test]
    #[should_panic(expected = "Env var not found")]
    fn test_new_write_path_missing_env_var() {
        // Ensure the environment variable is not set
        env::remove_var(NOTE_PATH);

        // This should panic since the environment variable is not set
        let _ = new_write_path("any title");
    }

    #[test]
    #[should_panic(expected = "Book title is empty!")]
    fn test_new_write_path_empty_title() {
        // Setup
        env::set_var(NOTE_PATH, "/tmp/notes/");

        // This should panic because the title is empty
        let _ = new_write_path("");

        // Cleanup
        env::remove_var(NOTE_PATH);
    }
}
