use std::fs;
use std::path::{Path};
use walkdir::WalkDir;
use clap::{Parser};

#[derive(Parser)]
#[command(version, about, long_about = None)]
struct Cli {
    /// Required: path to the directory
    //#[arg(short, long)]
    dir_path: String,

    /// Optional: limit value (integer between 20 and 100)
    #[arg(short = 'l', long = "limit", value_name = "LIMIT filename length", value_parser = clap::value_parser!(i8).range(20..=100))]
    limit: Option<i8>,

    /// Optional: verbosity - shows list of renamed files
    #[arg(short = 'v', 
        long = "verbose", 
        default_value_t = false)]
    verbose: bool,
    
    /// Optional: dry-run - does not rename files, only shows what will be renamed
    #[arg(short = 'd', 
        long = "dryrun", 
        default_value_t = false)]
    dryrun: bool,
}

#[derive(Debug)]
enum RenameResult {
    Success,    // File was renamed successfully
    NoAction,   // No action taken (filename was already correct)
    Error(String), // Error with a message
}

fn transform(s: &str) -> String {
    s.to_lowercase().chars()
        .map(|c: char| {
            if c == ' ' || c == '-' {
                '_'
            } else if c != '!' && 
                c != ',' && 
                c != '(' && 
                c != ')' && 
                c != '\'' && 
                c != '\"' && 
                c != '*' {
                    c // keep other chars
            } else {
                '\0'
            }
        })
        .filter(|&c| c != '\0')
        .collect()
}

fn rename_file(path: &Path, limit: i8, v: bool, d: bool) -> RenameResult {
    let filename = path.file_name().unwrap().to_str().unwrap();
    
    let mut new_filename = transform(filename);

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

        // if file with a new filename already exists - skip file
        if new_path.exists() {
            return RenameResult::Error(format!("Error renaming file '{}', file exists!", filename));
        }
        
        if !d {
            if let Err(e) = fs::rename(path, &new_path) {
                eprintln!("Error renaming file '{}': {}", filename, e);
                return RenameResult::Error(format!("Error renaming file '{}': {}", filename, e));
            } else {
                if v {
                    println!("Renamed: '{}' -> '{}'", filename, new_filename);
                }
                return RenameResult::Success; // Success
            }
        } else {
            println!("Will be renamed: '{}' -> '{}'", filename, new_filename);
        }
    }
    // If no renaming happened, return -1
    RenameResult::NoAction
}

fn process_directory(dir_path: &str, limit: i8, v: bool, d: bool) {
    let mut counter = 0;
    // Walk the directory recursively using WalkDir
    for entry in WalkDir::new(dir_path).into_iter().filter_map(Result::ok) {
        let path = entry.path();

        // Only rename files (not directories)
        if path.is_file() {
            match rename_file(path, limit, v, d) {
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

    let v = args.verbose;
    let d = args.dryrun;

    // Check if the directory exists
    if !Path::new(dir_path).exists() {
        eprintln!("Error: The directory '{}' does not exist.", dir_path);
        std::process::exit(1);
    }

    process_directory(dir_path, limit, v, d);
}
