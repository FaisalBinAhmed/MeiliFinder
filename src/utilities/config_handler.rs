// this file is used to read instances.json and settings.json files from the project root

use std::fs::File;
use serde::{Deserialize, Serialize};

use crate::app::Instance;

#[derive(Serialize, Deserialize)]
pub struct Settings {
    //tbd
}


pub fn retrieve_instances_from_file() -> Vec<Instance> {


    if let Ok(file) = File::open("instances.json") {

        let instances: Vec<Instance> = match serde_json::from_reader(file) {
            Ok(instances) => instances,
            Err(_) => vec![],
        };
        // println!("{:?}", instances);
        return instances;
    }
     vec![]

}

pub fn retrieve_settings_from_file() -> Settings {
    //tbd
    if let Ok(file) = File::open("settings.json") {
        let settings: Settings = match serde_json::from_reader(file) {
            Ok(settings) => settings,
            Err(_) => Settings{},
        };
        settings
    } else {
        Settings{}
    }
}