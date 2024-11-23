## Overview 🔍

`fs-rs` is a Rust-based command-line utility designed for performing efficient file searches within a specified directory. This tool currently supports searching for files by **exact names**, utilizing basic parallelism for improved performance in larger directories.

#### Features 🚀 
- **Exact Name Search**: The program allows searching for files with an exact name match.
- **Parallel Execution**: Leverages concurrency for faster performance, especially beneficial for directories with large numbers of files.

<br>

## Example usage 📂

```bash
fs-rs /path/to/search --name myfile.txt
```

This command will search the directory /path/to/search and find all occurrences of myfile.txt.

<br>

## Future Improvements 🚧

While fs-rs provides an efficient solution for exact name file searches, there are several areas planned for future improvements:

Support for File Types: Enable searching by file type or extension.
Partial Name Search: Add functionality to search for files based on partial name matches.
Performance Enhancements: Further optimizations to improve search speeds, particularly for very large file sets.
Improved Error Handling: Better handling of edge cases, such as inaccessible directories or invalid paths.

<br>

## License 📜

fs-rs is licensed under the MIT License.
