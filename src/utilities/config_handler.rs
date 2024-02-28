// this file is used to read instances.json and settings.json files from the project root

use std::fs::File;
use crate::app::Instance;



pub fn retrieve_instances_from_file() -> Vec<Instance> {


    if let Ok(file) = File::open("instances.json") {
        let reader = std::io::BufReader::new(file);
        let instances: Vec<Instance> = match serde_json::from_reader(reader) {
            Ok(instances) => instances,
            Err(_) => vec![],
        };
        return instances;
    }
     vec![]

}