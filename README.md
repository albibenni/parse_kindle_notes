## Why

This is a simple **Kindle** note parser to select _book title_ based notes, from the `.txt` file created from the Kindle.
It outputs a `.md` file with `---` spacers between each of them in the selected output folder, with the book title name.

## Setup

- Add `.env` file with a variable `NOTE_PATH` like:
  `NOTE_PATH="/Users/user/second_brain/books/"`
- Requires Rust 2021 or above
- Run with `cargo run` or build it with `cargo build` and run it `./target/debug/parse_kindle_notes` with 2 or 3 args depending on needs:
  - 2 args:
    1. **File name** to read from, must be of a supported extension (as of now `.txt`).
    2. **Book name** to parse notes from, eg. `the\ rust\ programming\ language`.
    - 3 args:
    1. **Folder name** where the file is located, eg. `Downloads/some`, to read from.
    2. **File name** to read from, must be of a supported extension (as of now `.txt`).
    3. **Book name** to parse notes from, eg. `the\ rust\ programming\ language`.
