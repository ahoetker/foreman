extern crate clap;
extern crate duma;
extern crate glob;
extern crate reqwest;
extern crate serde;
extern crate serde_json;
extern crate tempfile;

mod io;
mod modlist;
mod portal;
mod version;

use std::path::PathBuf;

use crate::portal::Portal;
use modlist::ModList;

fn main() {
    // retrieve the list of required mods
    let mod_list: ModList = match ModList::new("resources/mod-list-utility.json") {
        Ok(mod_list) => mod_list,
        Err(e) => {
            panic!("Could not parse mod list JSON: {}", e);
        }
    };

    // read the current contents of the "mods" directory into a vector
    let zip_files: Vec<PathBuf> = io::get_zips();

    // create a mod portal interface using the data from player-data.json
    let portal: Portal = Portal::new("resources/player-data.json").unwrap();

    // Download any missing or out of date mods
    mod_list
        .enabled_mods()
        .iter()
        .for_each(|m| match m.get_updated_listing(zip_files.clone()) {
            Ok(ml) => match ml {
                Some(ml) => {
                    let _completed_dl: PathBuf = portal.download_mod(ml).unwrap();
                    println!()
                }
                None => (),
            },
            _ => (),
        })
}
