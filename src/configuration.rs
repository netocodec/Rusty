extern crate serde;

use std::fs::File;
use std::fs;
use std::io::prelude::*;
use std::io::BufReader;
use serde::{Serialize, Deserialize};

#[path = "client_mem.rs"]
mod client_mem;


/// Filename of the configuration.
const FILENAME: &str = "config.conf";

/// Configuration memory structures and his default values.
static mut CONFIGURATION: FileContent = FileContent{
    port: 7005,
    max_clients: 10,
    connection_type: ConnectionType::TCP,
    blacklist_ips: []
};

/// Connections type
#[derive(Clone, Copy, Serialize, Deserialize, PartialEq, Debug)]
pub enum ConnectionType{
    /// TCP Connection
    TCP,
    /// UDP Connection
    UDP
}

/// File content structure
#[derive(Serialize, Deserialize, PartialEq, Debug)]
struct FileContent {
    /// Port of the server
    #[serde(default="default_port")]
    port: u16,
    /// Max clients of the server
    #[serde(default="default_max_clients")]
    max_clients: u16,
    /// Connection type of the server
    #[serde(default="default_connection")]
    connection_type: ConnectionType,
    /// Blacklist of the clients IP
    blacklist_ips: [String; 0]
}

/// Default port of the server
fn default_port() -> u16 {
    7005
}

/// Default max clients of the server
fn default_max_clients() -> u16 {
    10
}

/// Default connection type of the server
fn default_connection() -> ConnectionType {
    ConnectionType::TCP
}

/// This function will load all the configuration file data and puts into memory
/// If the file does not exists then it will create it and put the default data.
pub fn load_config() {
    let mut contents = String::new();
    let file = File::open(FILENAME);

    match file {
        Ok(file) => {
            let mut buf_reader = BufReader::new(file);
            buf_reader.read_to_string(&mut contents).unwrap();

            unsafe{
                CONFIGURATION = serde_json::from_str(&mut contents).unwrap();
            }
        },
        Err(_) => {
            match File::create(FILENAME) {
                Ok(mut fc) => {
                    unsafe{
                        let serialized = serde_json::to_string(&CONFIGURATION).unwrap();

                        fc.write_all(serialized.as_bytes()).unwrap();
                    }
                },
                Err(e) => panic!("Problem creating the file: {:?}", e),
            };        
        }
    };
}

/// Gets the port of the server.
pub fn get_port() -> u16 {
    unsafe {
        CONFIGURATION.port
    }
}

/// Gets the max clients of the server.
pub fn get_max_clients() -> usize {
    unsafe{
        CONFIGURATION.max_clients as usize
    }
}

/// Gets the connection type of the server.
pub fn get_connection_type() -> ConnectionType{
    unsafe{
        CONFIGURATION.connection_type
    }
}


/// This function will output the configuration data into the screen.
pub fn show_info_conf() {
    unsafe {
        println!("-------------------------");
        println!("------ Rusty V0.1 -------");
        println!("-------------------------");

        println!("IP: 0.0.0.0");
        println!("PORT: {}", CONFIGURATION.port);
        println!("MAX Clients: {}", CONFIGURATION.max_clients);
        println!("Connection Type: {:?}", CONFIGURATION.connection_type);
        println!("Number of blocked IPS: {}", CONFIGURATION.blacklist_ips.len());

        println!("-------------------------");
    }
}

/// This will check if the IP is on the blacklist or not.
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


/// This will check if the ip client is on the blacklist or the server reaches te maximum client
/// number.
pub fn is_user_allowed(ip: String) -> u8 {
    let mut result = 1;
    let is_blacklisted = check_ip_blacklist(&ip);
    let current_user_number = client_mem::get_connected_clients();
    let max_clients = get_max_clients();

    if is_blacklisted == 1 || current_user_number >= max_clients {
        result = 0;
    }

    result
}

#[cfg(test)]
mod tests{
    use super::*;

    #[test]
    fn test_read_file(){
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




