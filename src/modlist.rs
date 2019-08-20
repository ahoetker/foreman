use serde::Deserialize;
use std::error::Error;
use std::fs::File;
use std::io::BufReader;
use std::path::{Path, PathBuf};
use std::vec::Vec;

use crate::io::get_zips;
use crate::portal::ModListing;
use crate::version::Version;

#[derive(Clone, Deserialize, Debug)]
pub struct Mod {
    pub name: String,
    enabled: bool,
    version: Option<Version>,
}

impl Mod {
    pub fn get_installed_version(&self, mod_files: Vec<PathBuf>) -> Option<Version> {
        mod_files
            .into_iter()
            .filter(|f| String::from(f.to_str().unwrap()).contains(self.name.as_str()))
            .map(|f| Version::from(f))
            .max()
    }

    pub fn get_updated_listing(
        &self,
        mod_files: Vec<PathBuf>,
    ) -> Result<Option<ModListing>, Box<dyn std::error::Error>> {
        let mod_listing: ModListing = ModListing::new(&self.name)?;

        match self.get_installed_version(mod_files) {
            Some(v) => {
                let latest_version: Version = mod_listing.get_latest_version();
                println!(
                    "Mod: {:40}Installed: {:8}Available: {}",
                    self.name,
                    v.to_string(),
                    latest_version.to_string()
                );
                if latest_version > v {
                    Ok(Some(mod_listing))
                } else {
                    Ok(None)
                }
            }
            None => {
                println!(
                    "Mod: {:40}Installed: {:8}Available: {}",
                    self.name,
                    "None",
                    mod_listing.get_latest_version().to_string()
                );
                Ok(Some(mod_listing))
            }
        }
    }
}

impl std::fmt::Display for Mod {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mod_files: Vec<PathBuf> = get_zips();

        match self.get_installed_version(mod_files) {
            Some(version) => write!(f, "Mod: {:40} Version: {}", self.name, version.to_string()),
            None => write!(f, "Mod: {:40} Version: None", self.name),
        }
    }
}

#[derive(Deserialize, Debug)]
pub struct ModList {
    mods: Vec<Mod>,
}

impl ModList {
    pub fn new<P: AsRef<Path>>(path: P) -> Result<ModList, Box<dyn Error>> {
        let file = File::open(path)?;
        let reader = BufReader::new(file);
        let m = serde_json::from_reader(reader)?;
        Ok(m)
    }

    pub fn enabled_mods(&self) -> Vec<Mod> {
        self.mods
            .clone()
            .into_iter()
            .filter(|m| m.enabled == true)
            .collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new_modlist() {
        let mod_list: ModList = ModList::new("resources/mod-list-utility.json").unwrap();
        assert_eq!(mod_list.enabled_mods().len(), 7);
    }
}
