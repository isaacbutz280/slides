use regex::Regex;
use std::env;
use std::ffi::OsString;
use std::path::PathBuf;

#[derive(Debug)]
pub struct Config {
    pub format: String,
    pub extension: Option<OsString>,
    pub directory: PathBuf,
    pub operation: Operation,
}

#[derive(Debug)]
pub enum Operation {
    Prefix,
    Suffix,
    VersionRemove,
    Pattern,
}

static OPERATION_ERROR: &str = "Invalid operation input! Valid operations:\n \
   pre, prefix -> Adds the prefix specified to the files\n \
   suf, suffix -> Adds the suffix specified to the files\n \
   vr -> Removes the version tages that Windows automatically applies to \
repeated downloads\n \
  pat, pattern -> Takes the pattern passed in, and will replace * with \
numbers\n";

static FORMAT_ERROR: &str = "Improper pattern! Valid Patterns:\n \
   Prefix -> text followed by * and an optional file extension: pre_*.pdf\n \
   Suffix -> * followed by text and an optional file extension: *_suf\n \
   Version Removal -> * with an optional file extension: *, *.txt\n \
   Pattern -> Text containing * that will be replaced by a number, optional \
file extension: Lecture_*_.pptx, number*\n";

// A config will parse args, ensuring that a valid pattern
// is passed in, as well as a directory. If no directory is used,
// then the current working directory is passed
impl Config {
    pub fn new(mut args: env::Args) -> Result<Config, &'static str> {
        {
            args.next(); // Ignore the command call

            let operation = match args.next() {
                Some(s) if s == "pre" || s == "prefix" => Operation::Prefix,
                Some(s) if s == "suf" || s == "suffix" => Operation::Suffix,
                Some(s) if s == "vr" => Operation::VersionRemove,
                Some(s) if s == "pat" || s == "pattern" => Operation::Pattern,
                _ => return Err(OPERATION_ERROR),
            };

            let format = match args.next() {
                Some(arg) => {
                    is_valid_pattern(&arg, &operation)?;
                    arg
                }
                None => return Err(FORMAT_ERROR),
            };

            let extension: Option<OsString>;
            if let Some((_, e)) = format.rsplit_once('.') {
                extension = Some(OsString::from(e));
            } else {
                extension = None;
            }

            let directory = match args.next() {
                // Watch escaped quotations from ending on \
                Some(dir) => {
                    let p = PathBuf::from(dir);
                    if p.is_dir() {
                        p
                    } else {
                        return Err("Invalid directory! Please enter a valid directory\
                        \nNote: If your path has spaces, surrond with '', and \
                        ensure there is no trailing '\\' \n\
                        Example:'C:\\Users\\A User' ");
                    }
                }
                None => env::current_dir().expect("Failed to get CWD"),
            };

            Ok(Config {
                format,
                extension,
                directory,
                operation,
            })
        }
    }
}

fn is_valid_pattern(pat: &String, operation: &Operation) -> Result<(), &'static str> {
    match operation {
        Operation::Pattern => {
            let re = 
            Regex::new(r"^[^\.\*]*\*[^\.\*]*$|^[^\.\*]*\*[^\.\*]*\.\w+$")
            .unwrap();
            if re.is_match(&pat) {
                Ok(())
            } else {
                Err(FORMAT_ERROR)
            }
        }
        Operation::Prefix => {
            let re = Regex::new(r"^[^\.\*]+\*$|^[^\.\*]+\*\.\w+$").unwrap();
            if re.is_match(&pat) {
                Ok(())
            } else {
                Err(FORMAT_ERROR)
            }
        }
        Operation::Suffix => {
            let re = Regex::new(r"^\*[^\.\*]+$|^\*[^\.\*]+\.\w+$").unwrap();
            if re.is_match(&pat) {
                Ok(())
            } else {
                Err(FORMAT_ERROR)
            }
        }
        Operation::VersionRemove => {
            let re = Regex::new(r"^\*[\.\w]*$").unwrap();
            if re.is_match(&pat) {
                Ok(())
            } else {
                Err(FORMAT_ERROR)
            }
        }
    }
}
