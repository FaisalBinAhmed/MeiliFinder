use anyhow::Result;

use crate::{app::Instance, constants::APP_NAME_ASCII};

use std::io::{self, BufRead};


pub fn prompt_user_for_instance_info() -> Result<Instance> {

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

        Ok(Instance {
            id: "id".to_string(), //temp
            name: name.trim().to_string(),
            host: host.trim().to_string(),
            primary_key: primary_key.trim().to_string(),
    })

}

