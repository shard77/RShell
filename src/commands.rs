use std::fs;
use std::fs::File;
use std::path::Path;
use std::env;

pub(crate) fn ls(path: &str) {
    if let Ok(entries) = fs::read_dir(path) {
        for entry in entries {
            if let Ok(entry) = entry {
                if let Ok(metadata) = entry.metadata() {
                    if metadata.is_dir() {
                        println!("<DIR> {}", entry.file_name().to_str().unwrap())
                    } else {
                        println!("<FILE> {}", entry.file_name().to_str().unwrap())
                    }
                }
            }
        }
    }
}


pub (crate) fn cat(file_path: &str) {
    if let Ok(content) = fs::read_to_string(file_path) {
        println!("Cat Output:\n{}", content);
    } else {
        println!("Error: File not found");
    }
}


pub (crate) fn touch(file_name: &str) {
    if let Ok(file) = File::create(file_name) {
        println!("File '{}' has been created", file_name)
    } else {
        println!("Error with file creation");
    }
}

pub (crate) fn cd(path: &str) {
    let root = Path::new(path);
    match env::set_current_dir(&root) {
        Ok(()) => {
            println!("Successfully changed working directory to {}!", root.display());
        }
        Err(err) => {
            eprintln!("Error: {}", err);
        }
    }
}
