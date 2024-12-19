use std::env;
use std::fs;
use std::path::{Path};
use walkdir::WalkDir;
use clap::{Parser, Arg};

/*
rename all files under a given dir and subdirs using rules:
- lowercase filenames
- spaces, () replaced by _

fsren <path to dir>

rename all files under a given dir and subdirs using rules above +:
- limit filename to n characters
fsren <path to dir> -l <n>

TODO:
rename all files under a given dir and subdirs using rules above +:
- encrypt filenames with AES and base32
- key is randomly generated
fsren <path to dir> -e

rename all files under a given dir and subdirs using rules above +:
- encrypt files with AES and base32
- with a provided key
fsren <path to dir> -e -k <path to the key>

- generate random key in the given dir
fsren -g <path to the dir>

rename all files under a given dir and subdirs using rules above +:
- decrypt files with AES and base32
- with a provided key
fsren <path to dir> -d -k <path to the key>
*/

#[derive(Parser)]
#[command(version, about, long_about = None)]
struct Cli {
    /// Path to the directory
    //#[arg(short, long)]
    dir_path: String,

    /// Optional limit value (integer between 20 and 100)
    #[arg(short = 'l', long = "limit", value_name = "LIMIT filename length", value_parser = clap::value_parser!(i8).range(20..=100))]
    limit: Option<i8>,
}

#[derive(Debug)]
enum RenameResult {
    Success,    // File was renamed successfully
    NoAction,   // No action taken (filename was already correct)
    Error(String), // Error with a message
}

fn rename_file(path: &Path, limit: i8) -> RenameResult {
    let filename = path.file_name().unwrap().to_str().unwrap();
    
    // Convert the filename to lowercase and replace spaces with underscores
    // TODO: think about regex solution
    // let re = Regex::new(r"[ ()]").unwrap();
    // let replaced = re.replace_all((filename, "_"));
    let mut new_filename = filename.to_lowercase()
        .replace(" ", "_")
        .replace("(", "_")
        .replace(")", "_");

    // If the filename has an extension, split it and limit the base name length
    if let Some(extension) = path.extension() {
        let ext = extension.to_str().unwrap().to_lowercase(); // Get the extension in lowercase

        // Truncate the base filename to 30 characters
        let base_filename = new_filename.split('.').next().unwrap_or("");
        let truncated_base = if limit != 0 && base_filename.len() > limit as usize {
            base_filename.chars().take(30).collect::<String>()
        } else {
            base_filename.to_string()
        };

        // Combine the truncated base name with the extension
        new_filename = format!("{}.{}", truncated_base, ext);
    } else {
        // If there's no extension, just truncate the filename to 30 characters
        if new_filename.len() > 30 {
            new_filename = new_filename.chars().take(30).collect::<String>();
        }
    }

    // Only rename if the filename has changed
    if filename != new_filename {
        let new_path = path.with_file_name(new_filename.clone()); // Clone here to avoid moving

        if let Err(e) = fs::rename(path, &new_path) {
            eprintln!("Error renaming file '{}': {}", filename, e);
            return RenameResult::Error(format!("Error renaming file '{}': {}", filename, e));
        } else {
            println!("Renamed: '{}' -> '{}'", filename, new_filename);
            return RenameResult::Success; // Success
        }
    }
    // If no renaming happened, return -1
    RenameResult::NoAction
}

fn process_directory(dir_path: &str, limit: i8) {
    let mut counter = 0;
    // Walk the directory recursively using WalkDir
    for entry in WalkDir::new(dir_path).into_iter().filter_map(Result::ok) {
        let path = entry.path();

        // Only rename files (not directories)
        if path.is_file() {
            match rename_file(path, limit) {
                RenameResult::Success => counter += 1,
                RenameResult::NoAction => counter -= 0,
                RenameResult::Error(e) => eprintln!("Error: {}", e),
            }
        }
    }
    println!("Processed {counter} files");
}

fn main() {
    let args = Cli::parse();

    // If limit is not provided, override it to -1
    let limit = match args.limit {
        Some(value) => value,
        None => 0,
    };

    if limit == 0 {
        println!("Limit was not provided, using default: 0");
    }

    let dir_path = &args.dir_path;

    // Check if the directory exists
    if !Path::new(dir_path).exists() {
        eprintln!("Error: The directory '{}' does not exist.", dir_path);
        std::process::exit(1);
    }

    // Process the directory and its subdirectories
    process_directory(dir_path, limit);
}
