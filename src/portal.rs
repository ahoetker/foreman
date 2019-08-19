use crate::version::Version;
use serde::Deserialize;

struct Portal {
    // Stores information needed to use the mod portal API
    // Unlikely to implement API methods directly, those
    // are left to other structs
    token: String,
}

#[derive(Deserialize, Debug, Clone)]
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
        let mod_endpoint: String = format!("https://mods.factorio.com/api/mods/{}", name);
        let listing: ModListing = reqwest::get(&mod_endpoint)?.json()?;
        Ok(listing)
    }

    pub fn get_release_url(&self, version: Version) -> String {
        self.releases
            .iter()
            .filter(|release| release.version == version.to_string().as_str())
            .collect::<Vec<&Release>>()
            .first()
            .unwrap()
            .download_url
            .to_string()
    }

    pub fn get_latest_version(&self) -> Version {
        self.releases
            .clone()
            .into_iter()
            .map(|release| Version::from(release.version))
            .max()
            .unwrap()
    }

    pub fn get_latest_url(&self) -> String {
        self.get_release_url(self.get_latest_version())
    }
}
