use anyhow::Result;

use crate::{app::Instance, constants::APP_NAME_ASCII, utilities::config_handler::{check_if_instances_file_exists, save_instance_to_json_file}};

use std::io::{self, BufRead};


pub fn prompt_user_for_instance_info() -> Result<Instance> {


        //we should check whether there is an instances.json file already

        if check_if_instances_file_exists() {
            return Err(anyhow::anyhow!("instances.json file already exists"));
            // no need to prompt the user for instance info
        }

        let stdin = io::stdin();
        let mut handle = stdin.lock();

        // let mut id = String::new();
        let mut name = String::new();
        let mut host = String::new();
        let mut primary_key = String::new();

        println!("{}", APP_NAME_ASCII);

        println!("Enter the name of the instance:");
        handle.read_line(&mut name)?;

        println!("Enter the address of the instance:");
        handle.read_line(&mut host)?;

        println!("Enter the primary key of the instance:");
        handle.read_line(&mut primary_key)?;

        //save the instance info to instances.json
        let instance = Instance {
            id: "first_instance".to_string(), // we dont have to randomize it, as we'll only have one instance when user is prompted
            name: name.trim().to_string(),
            host: host.trim().to_string(),
            primary_key: primary_key.trim().to_string(),
    };

    save_instance_to_json_file(&instance)?;

    Ok(instance)

}

