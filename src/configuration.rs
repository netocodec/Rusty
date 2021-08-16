extern crate serde;

use std::fs::File;
use std::fs;
use std::io::prelude::*;
use std::io::BufReader;
use serde::{Serialize, Deserialize};


const FILENAME: &str = "config.conf";

static mut CONFIGURATION: FileContent = FileContent{
    port: Port(7005),
    max_clients: MaxClients(10),
    blacklist_ips: []
};

#[derive(Serialize, Deserialize, PartialEq, Debug)]
struct FileContent {
    #[serde(default)]
    port: Port,
    #[serde(default)]
    max_clients: MaxClients,
    blacklist_ips: [String; 0]
}

/// Port of the server.
#[derive(Serialize, Deserialize, PartialEq, Debug)]
struct Port(u16);
impl Default for Port {
    fn default() -> Self {
        Port(7005)
    }
}


/// Max Clients of the server.
#[derive(Serialize, Deserialize, PartialEq, Debug)]
struct MaxClients(u8);
impl Default for MaxClients {
    fn default() -> Self {
        MaxClients(10)
    }
}


pub fn load_config() {
    let mut contents = String::new();
    let file = File::open(FILENAME);

    match file {
        Ok(file) => {
            let mut buf_reader = BufReader::new(file);

            buf_reader.read_to_string(&mut contents);
        },
        Err(_) => {
            match File::create(FILENAME) {
                Ok(_fc) => {
                    unsafe{
                        let serialized = serde_json::to_string(&CONFIGURATION).unwrap();

                        println!("serialized = {}", serialized);

                    }
                },
                Err(e) => panic!("Problem creating the file: {:?}", e),
            };        
        }
    };
}



#[cfg(test)]
mod tests{
    use super::*;

    #[test]
    fn test_read_file(){
        println!("Done!");
        load_config();

        clean_file();
    }

    #[test]
    fn test_default_data(){
        unsafe{
            assert_eq!(Port(7005), CONFIGURATION.port);
            assert_eq!(MaxClients(10), CONFIGURATION.max_clients);
        }
    }

    #[test]
    fn test_serialize_data(){
        unsafe {
            let serialized_data = serde_json::to_string(&CONFIGURATION).unwrap();
            let struct_str = "{\"port\":7005,\"max_clients\":10,\"blacklist_ips\":[]}";

            println!("Default data = {}", struct_str);
            println!("Serialized data = {}", serialized_data);

            assert_eq!(serialized_data, struct_str);
        }
    }

    #[test]
    fn test_deserialize_data() {
        unsafe{
            let struct_str = "{\"port\":7005,\"max_clients\":10,\"blacklist_ips\":[]}";
            let deserialized_data: FileContent = serde_json::from_str(&struct_str).unwrap();

            println!("Default data = {}", struct_str);
            println!("Deserialized data = {:#?}", deserialized_data);

            assert_eq!(&deserialized_data, &CONFIGURATION);

        }
    }

    fn clean_file() {
        fs::remove_file(FILENAME);
    }
}




