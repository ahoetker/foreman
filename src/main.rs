extern crate glob;
extern crate reqwest;
extern crate serde;
extern crate serde_json;

use serde::Deserialize;
use std::error::Error;
use std::fs::File;
use std::io::BufReader;
use std::path::{Path, PathBuf};
use std::vec::Vec;

mod io;
mod portal;
mod version;
use version::Version;

#[derive(Clone, Deserialize, Debug)]
struct Mod {
    name: String,
    enabled: bool,
    // version: Option<(i8, i8, i8)>,
    version: Option<Version>,
}

impl Mod {
    fn get_installed_version(&self, mod_files: Vec<PathBuf>) -> Option<Version> {
        mod_files
            .into_iter()
            .filter(|f| String::from(f.to_str().unwrap()).contains(self.name.as_str()))
            .map(|f| Version::from(f))
            .max()
    }
}

impl std::fmt::Display for Mod {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mod_files: Vec<PathBuf> = io::get_zips();

        match self.get_installed_version(mod_files) {
            Some(version) => write!(f, "Mod: {:40} Version: {}", self.name, version.to_string()),
            None => write!(f, "Mod: {:40} Version: None", self.name),
        }
    }
}

#[derive(Deserialize, Debug)]
struct ModList {
    mods: Vec<Mod>,
}

impl ModList {
    fn new<P: AsRef<Path>>(path: P) -> Result<ModList, Box<dyn Error>> {
        let file = File::open(path)?;
        let reader = BufReader::new(file);
        let m = serde_json::from_reader(reader)?;
        Ok(m)
    }

    fn enabled_mods(&self) -> Vec<Mod> {
        self.mods
            .clone()
            .into_iter()
            .filter(|m| m.enabled == true)
            .collect()
    }
}

fn main() {
    let mod_list: ModList = ModList::new("resources/mod-list-ab.json").unwrap();

    mod_list
        .enabled_mods()
        .iter()
        .for_each(|m| println!("{}", m));

    // let oresilos = portal::ModListing::new("angelsaddons-oresilos").unwrap();
    // println!("{:?}", oresilos);
    //
    //
    // let silo_url: String = oresilos.get_release_url(Version::from((0, 5, 1)));
    // println!("{}", silo_url);
}
