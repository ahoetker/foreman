use regex::Regex;
use serde::Deserialize;
use std::cmp::Ordering;

#[derive(Deserialize, Debug, Eq, Clone)]
pub struct Version {
    version_tuple: (i8, i8, i8),
    version_str: String,
}

impl std::string::ToString for Version {
    fn to_string(&self) -> String {
        self.version_str.clone()
    }
}

impl From<(i8, i8, i8)> for Version {
    fn from(v: (i8, i8, i8)) -> Self {
        Version {
            version_tuple: v,
            version_str: format!("{}.{}.{}", v.0, v.1, v.2),
        }
    }
}

impl From<(&str)> for Version {
    fn from(s: &str) -> Self {
        let cap: regex::Captures = Regex::new(r"(\d+).(\d+).(\d+)")
            .unwrap()
            .captures(s)
            .unwrap();

        let v_tuple: (i8, i8, i8) = (
            cap[1].parse().unwrap(),
            cap[2].parse().unwrap(),
            cap[3].parse().unwrap(),
        );

        Version {
            version_tuple: v_tuple,
            version_str: String::from(s),
        }
    }
}

impl From<(std::string::String)> for Version {
    fn from(s: std::string::String) -> Self {
        let cap: regex::Captures = Regex::new(r"(\d+).(\d+).(\d+)")
            .unwrap()
            .captures(s.as_str())
            .unwrap();

        let v_tuple: (i8, i8, i8) = (
            cap[1].parse().unwrap(),
            cap[2].parse().unwrap(),
            cap[3].parse().unwrap(),
        );

        Version {
            version_tuple: v_tuple,
            version_str: s,
        }
    }
}

impl From<(std::path::PathBuf)> for Version {
    fn from(p: std::path::PathBuf) -> Self {
        let cap: regex::Captures = Regex::new(r"(\d+).(\d+).(\d+)")
            .unwrap()
            .captures(p.to_str().unwrap())
            .unwrap();

        let v_tuple: (i8, i8, i8) = (
            cap[1].parse().unwrap(),
            cap[2].parse().unwrap(),
            cap[3].parse().unwrap(),
        );

        Version {
            version_tuple: v_tuple,
            version_str: format!("{}.{}.{}", v_tuple.0, v_tuple.1, v_tuple.2),
        }
    }
}

impl Ord for Version {
    fn cmp(&self, other: &Self) -> Ordering {
        self.version_tuple.cmp(&other.version_tuple)
    }
}

impl PartialOrd for Version {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl PartialEq for Version {
    fn eq(&self, other: &Self) -> bool {
        self.version_tuple == other.version_tuple
    }
}
