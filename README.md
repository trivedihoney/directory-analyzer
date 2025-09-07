# 📂 Directory Analyzer

A simple Rust CLI tool to scan a directory and export file metadata (path and last modified date) to a CSV file.

## Features

- Recursively scans a given folder for files
- Collects file paths and last modified timestamps
- Exports the data to a customizable CSV file
- Progress updates in the terminal
- User-friendly prompts and colored output

## Usage

1. **Build the project:**

   ```bash
   cargo build --release
   ```

2. **Run the tool:*

   ```bash
   ./target/release/directory-analyzer
   ```

3. **Follow the prompts:**
   - Enter the folder path to scan (default: `./`)
   - Enter the output CSV filename (default: `file_data.csv`)

4. **Result:**
   - The tool will scan all files in the directory, display progress, and export the results to the specified CSV file.

## Example Output

``` txt
Path,Modified On
/home/user/projects/file1.txt,2025-09-07 12:34:56
/home/user/projects/file2.rs,2025-09-06 09:21:10
...
```

## Dependencies

- [anyhow](https://crates.io/crates/anyhow)
- [chrono](https://crates.io/crates/chrono)
- [colored](https://crates.io/crates/colored)
- [csv](https://crates.io/crates/csv)
- [walkdir](https://crates.io/crates/walkdir)

Install dependencies with Cargo (handled automatically).
