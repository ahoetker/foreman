use regex::Regex;
use std::path::PathBuf;

pub struct Version {
    version_tuple: Option<(i8, i8, i8)>,
    version_str: Option<String>,
}

impl Version {
    pub fn version_from_path(path: PathBuf) -> Option<(i8, i8, i8)> {
        let cap: regex::Captures = Regex::new(r"(\d+).(\d+).(\d+)")
            .unwrap()
            .captures(path.to_str()?)
            .unwrap();

        Some((
            cap[1].parse().unwrap(),
            cap[2].parse().unwrap(),
            cap[3].parse().unwrap(),
        ))
    }

    pub fn version_from_str(s: String) -> Option<(i8, i8, i8)> {
        let cap: regex::Captures = Regex::new(r"(\d+).(\d+).(\d+)")
            .unwrap()
            .captures(s.as_str())
            .unwrap();

        Some((
            cap[1].parse().unwrap(),
            cap[2].parse().unwrap(),
            cap[3].parse().unwrap(),
        ))
    }
}

impl std::string::ToString for Version {
    fn to_string(&self) -> String {
        match &self.version_str {
            Some(version) => format!("{}", version),
            None => format!(
                "{}.{}.{}",
                self.version_tuple.unwrap().0,
                self.version_tuple.unwrap().1,
                self.version_tuple.unwrap().2
            ),
        }
    }
}
