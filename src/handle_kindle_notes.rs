pub fn parse_kindle_notes(path: &str, book_title: &str) {
    println!("{}", path);
    let file: String = std::fs::read_to_string(path).expect("No file found.");

    let repl: &String = &file.replace("\u{feef}", "");
    let lines: &Vec<&str> = &repl.split("\n").collect();
    let parsed_file = parse_file_by_book_title(book_title, lines);

    for l in parsed_file {
        println!("LOL: {:?}", l);
    }
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
    return parsed_file;
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
}
