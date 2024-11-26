use regex::Regex;
use std::collections::{HashMap, HashSet};
use std::fs::{self, File, OpenOptions};
use std::io::{self, BufRead, Write};
use std::path::Path;
use std::process::{exit, Command};

// Create a prototype requirement file to track current requirements
// Deleted on completion
pub fn _install() -> i32 {
    let _pre = Command::new("touch")
        .arg("pro_requirements.txt")
        .output()
        .expect("Failed at creating initial repository for your requirements.");

    if _pre.status.success() {
        let _create = Command::new("sh")
            .args(["-c", "pip3 list > pro_requirements.txt"])
            .output()
            .expect("Failed at initializing pro_requirements.txt");

        if !_create.status.success() {
            return 0;
        }
    }

    1
}

// reads from pro_requirements.txt to a vector to keep track
// new: uses a HashMap to store package names and versions
pub fn _requirements() -> HashMap<String, String> {
    let path = "pro_requirements.txt";
    let pattern = Regex::new(r"^[a-zA-Z0-9_]+").unwrap();

    let mut requires_v_details = HashMap::new();

    if let Ok(file) = File::open(path) {
        for line in io::BufReader::new(file).lines() {
            if let Ok(req) = line {
                let trimmed = req.trim();

                if let Some(found) = pattern.find(trimmed) {
                    if found.as_str() != "Package" {
                        let temp: Vec<&str> = trimmed.split_whitespace().collect();

                        if temp.len() == 2 {
                            requires_v_details.insert(temp[0].to_string(), temp[1].to_string());
                        }
                    }
                }
            }
        }
    } else {
        eprintln!("Failed to open file: {}", path);
        return HashMap::new();
    }

    requires_v_details
}

// Remove entries that do not exist in the given Python file
// should work fine because 'internal file names' do not belong to requirements.txt
// setuptools, pip
// transient dependencies are resolved by main dependency
// installing flask would itself install sqlparse, asgi, etcetera.
pub fn _pop(requires: &mut HashMap<String, String>, paths: &Vec<String>) {
    let mut content: HashSet<String> = HashSet::new();

    for path in paths {
        let file = match File::open(path) {
            Ok(file) => file,
            Err(_) => {
                eprintln!("Could not open file: {}", path);
                continue;
            }
        };
        let reader = io::BufReader::new(file);

        content.extend(
            reader
                .lines()
                .filter_map(|line| line.ok())
                .flat_map(|line| {
                    line.split_whitespace()
                        .map(|word| word.to_lowercase())
                        .collect::<Vec<String>>() 
                })
        );
    }

    requires.retain(|key, _| content.contains(&key.to_lowercase()));
}

pub fn cigarettes(requires: &HashMap<String, String>, path: &str) -> io::Result<()>{

    let mut _file = File::create(path)?;
    for (name, version) in requires {
        writeln!(_file, "{}=={}", name, version);
    }

    _file.flush()?;

    Ok(())
}