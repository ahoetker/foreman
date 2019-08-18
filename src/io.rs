use std::path::Path;
use std::path::PathBuf;
use std::vec::Vec;

pub fn all_files(path: &Path) -> Vec<PathBuf> {
    let mut files: Vec<PathBuf> = vec![];
    for entry in glob::glob(format!("{}/*.zip", path.display()).as_str()).unwrap() {
        match entry {
            Ok(p) => files.push(p),
            Err(err) => panic!("Glob Error: {}", err),
        }
    }
    files
}

pub fn get_zips() -> Vec<PathBuf> {
    all_files(Path::new("mods"))
}
