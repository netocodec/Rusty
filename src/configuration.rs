use std::fs::File;
use std::fs;
use std::io::prelude::*;
use std::io::{BufReader, Result, ErrorKind};

const FILENAME: &str = "config.conf";


struct FileContent {
    port: u16,
    max_clients: u8,
    blacklist_ips: [String; 128]
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
                Ok(fc) => fc,
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


    fn clean_file() {
        fs::remove_file(FILENAME);
    }
}




