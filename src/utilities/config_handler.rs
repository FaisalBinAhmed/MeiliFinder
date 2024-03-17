// this file is used to read instances.json and settings.json files from the project root

use std::fs::File;
use anyhow::Result;
use serde::{Deserialize, Serialize};

use crate::{app::app::Instance, constants::INSTANCE_FILE_NAME};

#[derive(Serialize, Deserialize)]
pub struct Settings {
    //tbd
}


pub fn check_if_instances_file_exists() -> bool {

    if let Ok(file) = File::open(INSTANCE_FILE_NAME) {

        let instances: Vec<Instance> = match serde_json::from_reader(file) {
            Ok(instances) => instances,
            Err(_) => vec![],
        };
        if instances.len() > 0 {
            return true;
        }
    }
     false
    
}


pub fn save_instance_to_json_file(instance: &Instance) -> Result<()> {
    let json = serde_json::to_string_pretty(&[instance])?;
    std::fs::write(INSTANCE_FILE_NAME, json)?; //stores this file in the root directory

    Ok(())
}

pub fn retrieve_instances_from_file() -> Vec<Instance> {


    if let Ok(file) = File::open(INSTANCE_FILE_NAME) {

        let instances: Vec<Instance> = match serde_json::from_reader(file) {
            Ok(instances) => instances,
            Err(_) => vec![],
        };
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