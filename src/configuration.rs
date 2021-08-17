extern crate serde;

use std::fs::File;
use std::fs;
use std::io::prelude::*;
use std::io::BufReader;
use serde::{Serialize, Deserialize};


const FILENAME: &str = "config.conf";

static mut CONFIGURATION: FileContent = FileContent{
    port: 7005,
    max_clients: 10,
    blacklist_ips: []
};

#[derive(Serialize, Deserialize, PartialEq, Debug)]
struct FileContent {
    #[serde(default="default_port")]
    port: u16,
    #[serde(default="default_max_clients")]
    max_clients: u16,
    blacklist_ips: [String; 0]
}

fn default_port() -> u16 {
    7005
}

fn default_max_clients() -> u16 {
    10
}


pub fn load_config() {
    let mut contents = String::new();
    let file = File::open(FILENAME);

    match file {
        Ok(file) => {
            let mut buf_reader = BufReader::new(file);
            buf_reader.read_to_string(&mut contents);

            unsafe{
                CONFIGURATION = serde_json::from_str(&mut contents).unwrap();
            }
        },
        Err(_) => {
            match File::create(FILENAME) {
                Ok(mut fc) => {
                    unsafe{
                        let serialized = serde_json::to_string(&CONFIGURATION).unwrap();

                        fc.write_all(serialized.as_bytes());
                    }
                },
                Err(e) => panic!("Problem creating the file: {:?}", e),
            };        
        }
    };
}

pub fn get_port() -> u16 {
    unsafe {
        CONFIGURATION.port
    }
}

pub fn get_max_clients() -> usize {
    unsafe{
        CONFIGURATION.max_clients as usize
    }
}


pub fn show_info_conf() {
    unsafe {
        println!("-------------------------");
        println!("---- Rusty V0.1 BETA ----");
        println!("-------------------------");


        println!("IP: 0.0.0.0");
        println!("PORT: {}", CONFIGURATION.port);
        println!("MAX Clients: {}", CONFIGURATION.max_clients);
        println!("Number of blocked IPS: {}", CONFIGURATION.blacklist_ips.len());

        println!("-------------------------");
    }
}


pub fn check_ip_blacklist(ip: &String) -> u8 {
    let mut result = 0;

    unsafe {
        for current_ip in &CONFIGURATION.blacklist_ips{
            if current_ip == ip {
                result = 1;
                break;
            }
        }
    }

    result
}

#[cfg(test)]
mod tests{
    use super::*;

    #[test]
    fn test_read_file(){
        println!("Done!");
        load_config();
    }

    #[test]
    fn test_default_data(){
        unsafe{
            assert_eq!(7005, CONFIGURATION.port);
            assert_eq!(10, CONFIGURATION.max_clients);
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

    #[test]
    fn clean_file() {
        fs::remove_file(FILENAME);
    }
}




