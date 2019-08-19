extern crate clap;
extern crate duma;
extern crate glob;
extern crate rayon;
extern crate reqwest;
extern crate serde;
extern crate serde_json;
extern crate tempfile;

use rayon::prelude::*;
use serde::Deserialize;
use std::error::Error;
use std::fs::File;
use std::io::BufReader;
use std::path::{Path, PathBuf};
use std::vec::Vec;

mod io;
mod portal;
mod version;
use crate::portal::Portal;
use version::Version;

#[derive(Clone, Deserialize, Debug)]
struct Mod {
    name: String,
    enabled: bool,
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
    let mod_list: ModList = match ModList::new("resources/mod-list-utility.json") {
        Ok(mod_list) => mod_list,
        Err(e) => {
            panic!("Could not parse mod list JSON: {}", e);
        }
    };

    // print out list of enabled mods and installed version
    //    mod_list
    //        .enabled_mods()
    //        .iter()
    //        .for_each(|m| println!("{}", m));

    let portal: Portal = Portal::new("resources/player-data.json").unwrap();

    mod_list.enabled_mods().iter().for_each(|m| {
        match portal::ModListing::new(&m.name) {
            Ok(ml) => {
                let completed_dl: PathBuf = portal.download_mod(ml).unwrap();
                println!("Download completed: {}\n", completed_dl.to_str().unwrap())
            }
            Err(e) => println!("Could not create mod listing: {}", e),
        };
    })
}
