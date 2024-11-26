use std::env;
use std::fs::{self, File};

mod pip4_0;
use pip4_0::*;

fn main() {

    
    
    if _install() == 0 {
        eprintln!("Failed to install requirements.");
        return;
    }

    let mut requirements = _requirements();
    let size_b4 = requirements.len();


    // If you resist adding any arguments current directory will be used
    let args: Vec<String> = env::args().collect();
    let directory = if args.len() > 1 {
        &args[1]  
    } else {
        "."  
    };
    
    let mut files: Vec<String> = Vec::new();

    if let Ok(entries) = fs::read_dir(directory) {
        for entry in entries.filter_map(Result::ok) {
            let path = entry.path();
            if path.extension().map(|ext| ext == "py").unwrap_or(false) {
                files.push(path.display().to_string());
            }
        }
    }


    let write_to = format!("{}/requirements.txt", directory);

    // Pops any requirement you will not need
    _pop(&mut requirements, &files);

    let size_after = requirements.len();

    // writes to requirements in directory = args[1]
    cigarettes(&requirements, &write_to);

    // delete pro_requirements.txt
    if let Err(e) = fs::remove_file("pro_requirements.txt") {
        eprintln!("Failed to delete {}: {}", "pro_requirements.txt", e);
    }
}
