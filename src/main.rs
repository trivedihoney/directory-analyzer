use anyhow::Result;
use chrono::{DateTime, Local};
use colored::*;
use csv::WriterBuilder;
use std::{
    fs::File,
    io::{self, Write},
    path::PathBuf,
    time::SystemTime,
};
use walkdir::WalkDir;

struct FileInfo {
    path: PathBuf,
    modified_on: SystemTime,
}

fn export_to_csv(data: &[FileInfo], output: &PathBuf) -> Result<()> {
    let file = File::create(output)?;
    let mut wtr = WriterBuilder::new().from_writer(file);

    wtr.write_record(["Path", "Modified On"])?;
    for info in data {
        let datetime: DateTime<Local> = info.modified_on.into();
        let formatted_time = datetime.format("%Y-%m-%d %H:%M:%S").to_string();
        wtr.write_record([&info.path.to_string_lossy(), formatted_time.as_str()])?;
    }
    wtr.flush()?;
    Ok(())
}

fn main() -> Result<()> {
    println!("{}", "📂 File Metadata Exporter".bold().blue());
    println!("Welcome! This tool scans a folder and exports file metadata to CSV.\n");

    // Ask for folder path
    print!("👉 Enter the folder path to scan (default = ./): ");
    io::stdout().flush()?;
    let mut dir_in = String::new();
    io::stdin().read_line(&mut dir_in)?;
    let dir = {
        let s = dir_in.trim();
        if s.is_empty() { PathBuf::from("./") } else { PathBuf::from(s) }
    };

    // Ask for output file
    print!("👉 Enter output CSV filename (default = file_data.csv): ");
    io::stdout().flush()?;
    let mut out_in = String::new();
    io::stdin().read_line(&mut out_in)?;
    let output = {
        let s = out_in.trim();
        if s.is_empty() { PathBuf::from("file_data.csv") } else { PathBuf::from(s) }
    };

    println!("\n🔍 Scanning {} ...", dir.display());

    let mut file_info_vec: Vec<FileInfo> = Vec::new();
    let mut counter: u64 = 0;

    for entry in WalkDir::new(&dir).into_iter().filter_map(|e| e.ok()) {
        if entry.file_type().is_file() {
            if let Ok(metadata) = entry.metadata() {
                if let Ok(modified_time) = metadata.modified() {
                    file_info_vec.push(FileInfo {
                        path: entry.path().to_path_buf(),
                        modified_on: modified_time,
                    });
                }
            }
            counter += 1;
            // overwrite the same line with \r
            print!("\r📄 Processed files: {}", counter);
            io::stdout().flush()?;
        }
    }
    println!("\n✅ Scan complete. Total files: {}", counter);

    // Sort and export
    file_info_vec.sort_by(|a, b| a.modified_on.cmp(&b.modified_on));
    match export_to_csv(&file_info_vec, &output) {
        Ok(_) => println!(
            "{} {}",
            "✅ Successfully exported data to".green(),
            output.display()
        ),
        Err(e) => eprintln!("{} {}", "❌ Error exporting to CSV:".red(), e),
    }

    Ok(())
}
