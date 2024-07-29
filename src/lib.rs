pub mod config;
pub mod walker;

#[macro_use]
extern crate lazy_static;
pub use crate::config::{Config, Operation};
pub use crate::walker::Walker;
use regex::Regex;
use std::error::Error;
use std::fs;
use std::path::PathBuf;
use std::ffi::OsStr;

// Called by main, takes a config and executes the code
pub fn run(c: &Config) -> Result<(), Box<dyn Error>> {
    /* My first attempt, wanted more flexibility so abstracted to Walker
    {
            // read_dir gives directory
    // Unwrap, we ensure it is valid in Config
    // Map each entry to a PathBuf
    // Filter each Pathbuf based on extension
    // Collect into vec
    let mut files: Vec<PathBuf> = c
    .directory
    .read_dir()
    .unwrap()
    .map(|entry| entry.unwrap().path())
    .filter(|p| match p.extension() {
        Some(ext) if ext == &c.extension.unwrap() => true,
        _ => false,
    })
    .collect();
    }
    */

    let walker = Walker::new(c.directory.clone(), false).walk();

    let files: Vec<PathBuf> = match &c.extension {
        Some(c_ext) => walker
            .filter(|path| match path.extension() {
                Some(p_ext) if p_ext == c_ext => true,
                _ => false,
            })
            .collect(),
        None => walker.collect::<Vec<PathBuf>>(),
    };

    // Check we don't remove anything important, like the program itself!
    let mut files = files
        .into_iter()
        .filter(|path| !path.extension().unwrap_or(OsStr::new("")).eq("exe"))
        .collect();

    replace(&mut files, &c)
}

// The actual logic of the code. Given a list of files and a configuration, 
// will iterate over files, and apply operation

// Possible improvement, redo this
fn replace(
    files: &mut Vec<PathBuf>,
    config: &Config
) -> Result<(), Box<dyn Error>> {

    // General defintions
    lazy_static! {
        static ref RE_PAT: Regex = Regex::new(r"[^\d]+([\d]{1,2})$|^([\d]{1,2})[^\d]+$|[^\d]+([\d]{1,2})[^\d]+|^([\d]{1,2})$").unwrap();
    }
    lazy_static! {
        static ref RE_VR: Regex = Regex::new(r" \(\d+\)").unwrap();
    }

    let operation = &config.operation;
    let extension = &config.extension;
    let format = &config.format;

    for file in files {
        let parts = (file.file_stem(), file.extension());
        if let (Some(stem), Some(ext)) = parts {
            if let (Some(stem), Some(ext)) = (stem.to_str(), ext.to_str()) {
                let mut new_name = match operation {
                    Operation::Prefix
                    | Operation::Suffix => format.replace('*', stem),

                    Operation::VersionRemove => {
                        let t = RE_VR.replace(stem, "").into_owned();
                        format.replace('*', t.as_str())
                    }
                    Operation::Pattern => {
                        if let Some(caps) = RE_PAT.captures(stem) {
                            let mut num = "";

                            //let mut t = caps.iter().skip(1);
                            for cap in caps.iter().skip(1) {
                                if let Some(cap) = cap {
                                    num = cap.as_str();                                    
                                }
                            }

                            format.replace('*', num)
                        } else {
                            continue; // Doesn't have a number in it, continue
                        }
                    }
                };

                // If the pattern doesn't contain an extension tack on now
                if let None = extension {
                    new_name.push('.');
                    new_name.push_str(ext);
                }

                rename(file, &new_name)?
            } // Either stem or ext invalid, oh well
        } // Path must not be valid, oh well
    }
    // No errors returned, all Ok!
    Ok(())
}


// Unused for now
fn _next_some<T>(iter: Box<dyn Iterator<Item = Option<T>>>, start: usize) -> Option<T> {
    let temp = iter.skip(start);
    for next in temp {
        if let Some(ret) = next {
            return Some(ret)
        }
    }

    // Made it through, no Some. 
    None
}

/// Given an old path and a new name, renames the file
fn rename(path: &PathBuf, new_name: &str) -> Result<(), Box<dyn Error>> {
    let mut new_path = path.clone();
    new_path.set_file_name(new_name);

    if new_path.exists() {
        // Path already exists, skip
        println!("Path already exists: {}\nSkipping...\n", new_path.display());
        Ok(())
    } else {
        println!("Renaming: {}\nTo: {}\n", path.display(), new_path.display());
        Ok(fs::rename(path, new_path)?)    
    }
}
