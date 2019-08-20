use clap::{clap_app, crate_version};
use duma::download::http_download;
use reqwest::Url;
use serde::Deserialize;
use serde_json::Value;
use std::fs::File;
use std::io::BufReader;
use std::path::Path;
use std::path::PathBuf;

use crate::version::Version;
use crate::modlist::Mod;

#[derive(Deserialize, Debug)]
pub struct Portal {
    // Stores information needed to use the mod portal API
    // Unlikely to implement API methods directly, those
    // are left to other structs
    token: String,
    username: String,
}

impl Portal {
    pub fn new<P: AsRef<Path>>(path: P) -> Result<Portal, Box<dyn std::error::Error>> {
        let file: File = File::open(path)?;
        let buf: BufReader<File> = BufReader::new(file);
        let portal: Value = serde_json::from_reader(buf)?;
        Ok(Portal {
            token: portal["service-token"].to_string().replace('"', ""),
            username: portal["service-username"].to_string().replace('"', ""),
        })
    }

    pub fn download_mod(&self, m: ModListing) -> Result<PathBuf, Box<dyn std::error::Error>> {
        let base: Url = Url::parse("https://mods.factorio.com")?;
        let url: Url = base.join(&m.get_latest_url())?;
        let download_url: Url = Url::parse_with_params(
            url.as_str(),
            &[("username", &self.username), ("token", &self.token)],
        )?;

        let p: PathBuf = PathBuf::from(format!(
            "mods/{}_{}.zip",
            m.name,
            m.get_latest_version().to_string()
        ));

        let args = clap_app!(Duma =>
    (version: crate_version!())
    (author: "Matt Gathu <mattgathu@gmail.com>")
    (about: "A minimal file downloader")
    (@arg quiet: -q --quiet "quiet (no output)")
    (@arg continue: -c --continue "resume getting a partially-downloaded file")
    (@arg singlethread: -s --singlethread "download using only a single thread")
    (@arg headers: -H --headers "prints the headers sent by the HTTP server")
    (@arg FILE: -O --output +takes_value "write documents to FILE")
    (@arg AGENT: -U --useragent +takes_value "identify as AGENT instead of Duma/VERSION")
    (@arg SECONDS: -T --timeout +takes_value "set all timeout values to SECONDS")
    (@arg NUM_CONNECTIONS: -n --num_connections +takes_value "maximum number of concurrent connections (default is 8)")
    (@arg URL: +required +takes_value "url to download")
    )
            .get_matches_from(vec!["duma", download_url.as_str(), "-O", &p.to_str().unwrap()]);

        http_download(download_url, &args, crate_version!())?;
        Ok(p)
    }
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
        #[cfg(test)]
        let base: Url = Url::parse(&mockito::server_url())?;

        #[cfg(not(test))]
        let base: Url = Url::parse("https://mods.factorio.com/api/mods/")?;

        let mod_endpoint: Url = base.join(name)?;
        let listing: ModListing = reqwest::get(mod_endpoint)?.json()?;
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
            .iter()
            .map(|release| Version::from(release.version.as_str()))
            .max()
            .unwrap()
    }

    pub fn get_latest_url(&self) -> String {
        self.get_release_url(self.get_latest_version())
    }
}

impl From<Mod> for ModListing {
    fn from(m: Mod) -> Self {
        ModListing::new(&m.name).unwrap()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use mockito::mock;
    use std::fs::read_to_string;
    use std::io::Write;
    use tempfile::{tempdir, TempDir};

    fn setup() -> mockito::Mock {
        mock("GET", "/Bottleneck")
            .with_status(201)
            .with_header("content-type", "application/json")
            .with_body::<String>(
                read_to_string("resources/bottleneck-test.json")
                    .unwrap()
                    .parse()
                    .unwrap(),
            )
            .create()
    }

    #[test]
    fn test_new_portal() {
        let dir: TempDir = tempdir().unwrap();
        let file_path: PathBuf = dir.path().join("test-player-data.json");
        let mut file = File::create(&file_path).unwrap();
        write!(
            file,
            r#"
            {{
                "service-username": "j_appleseed",
                "service-token": "1a2b3c4d5e"
            }}"#
        )
            .unwrap();

        let test_portal: Portal = Portal::new(&file_path).unwrap();
        assert_eq!(test_portal.username, "j_appleseed");
        assert_eq!(test_portal.token, "1a2b3c4d5e");

        drop(file);
        dir.close().unwrap();
    }

    #[test]
    fn test_new_modlisting() {
        let _m: mockito::Mock = setup();

        let mod_listing: ModListing = ModListing::new("Bottleneck").unwrap();

        assert_eq!(mod_listing.name, "Bottleneck");
        assert_eq!(mod_listing.title, "Bottleneck");
        assert_eq!(
            mod_listing.summary,
            "A tool for locating input starved machines."
        );
    }

    #[test]
    fn test_modlisting_from_mod() {
        let test_mod: Mod = serde_json::from_str(r#"
            {
                "name": "Bottleneck",
                "enabled": true
            }"#).unwrap();

        let mod_listing: ModListing = ModListing::from(test_mod);

        assert_eq!(mod_listing.name, "Bottleneck");
        assert_eq!(mod_listing.title, "Bottleneck");
        assert_eq!(
            mod_listing.summary,
            "A tool for locating input starved machines."
        );
    }

    #[test]
    fn test_get_latest_version() {
        let file: File = File::open("resources/releases-test.json").unwrap();
        let buf: BufReader<File> = BufReader::new(file);
        let releases: Vec<Release> = serde_json::from_reader(buf).unwrap();

        let mod_listing: ModListing = ModListing {
            name: String::from("test-mod"),
            summary: String::from("This is a test mod"),
            title: String::from("Test Mod"),
            releases: releases,
        };

        assert_eq!(mod_listing.get_latest_version(), Version::from("0.10.4"));
    }

    #[test]
    fn test_get_latest_url() {
        let _m: mockito::Mock = setup();
        let mod_listing: ModListing = ModListing::new("Bottleneck").unwrap();

        assert_eq!(&mod_listing.get_latest_url(), "/download/Bottleneck/5cc20d63e4ed41000b88d4d9");
    }
}
