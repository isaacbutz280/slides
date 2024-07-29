use std::path::PathBuf;

pub struct Walker {
    recursive: bool,
    directory: PathBuf,
}

// A walker is simply a way to iterate over all files in a directory,
// and return the PathBuf of each one.
impl Walker {

    // A simple constructor. For this project, the directory is validated
    // in config. In the future, should validate here
    pub fn new(directory: PathBuf, recursive: bool) -> Walker {
        Walker {
            recursive,
            directory,
        }
    }

    // Will iterate over the files in the directory that walker holds
    pub fn walk(&self) -> Box<dyn Iterator<Item = PathBuf>> {
        if self.recursive {
            let mut files: Vec<PathBuf> = vec![];
            recurse(&self.directory, &mut files);
            Box::new(files.into_iter())
        } else {
            Box::new(
                self.directory
                    .read_dir()
                    .unwrap()
                    .map(|entry| entry.unwrap().path())
                    .filter(|predicate| ! predicate.is_dir())
            )
        }
    }

    // Possible improvement: Add more methods, like this
    pub fn change_dir(& mut self, _new_path: &PathBuf) -> Option<()> {
        unimplemented!()
    }
}

// A helper method, uses the stack to iterate through directories (Depth First)
fn recurse(path: &PathBuf, files: &mut Vec<PathBuf>) {
    for entry in path.read_dir().expect("Read Dir failed") {
        if let Ok(entry) = entry {
            if entry.path().is_dir() {
                // Create subpath
                let mut new_path = path.clone();
                new_path.push(entry.file_name());

                recurse(&new_path, files);

            } else {
                // A file
                files.push(entry.path());
            }
        } // Invalid DirEntry
    }
}

