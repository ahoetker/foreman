use regex::Regex;
use serde::Deserialize;

struct Portal {
    // Stores information needed to use the mod portal API
    // Unlikely to implement API methods directly, those
    // are left to other structs
    token: String,
}

#[derive(Deserialize, Debug)]
pub struct Release {
    version: String,
    sha1: String,
    file_name: String,
    download_url: String,
}

#[derive(Deserialize, Debug)]
pub struct ModListing {
    name: String,
    summary: String,
    title: String,
    releases: Vec<Release>,
}

impl ModListing {
    pub fn new(name: &str) -> Result<ModListing, Box<dyn std::error::Error>> {
        let mod_endpoint = format!("https://mods.factorio.com/api/mods/{}", name);
        let listing: ModListing = reqwest::get(&mod_endpoint)?.json()?;
        Ok(listing)
    }

    pub fn get_release_url(&self, version: (i8, i8, i8)) -> String {
        let release_string: String = format!("{}.{}.{}", version.0, version.1, version.2);

        self.releases
            .iter()
            .filter(|release| release.version == release_string)
            .collect::<Vec<&Release>>()
            .first()
            .unwrap()
            .download_url
            .to_string()
    }
}
