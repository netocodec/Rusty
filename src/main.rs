use std::net::{TcpListener, TcpStream};
use std::io::Read;
use std::thread;

mod client_mem;
mod configuration;


fn handle_client(mut client: TcpStream){
    let mut data_msg = [0 as u8; 50];

    while match client.read(&mut data_msg){
        Ok(size) => {
            if size != 0 {
                let ip = client_mem::get_client_ip(&mut client);

                client_mem::send_message_to_clients(ip, &data_msg[0..size]);
                true
            }else{
                client_mem::shutdown_client(&mut client);
                false
            }
        },
            Err(_) => {
                println!("Error on client connection!");
                client_mem::shutdown_client(&mut client);
                false
            }
    }{}
}

fn is_user_allowed(ip: String) -> u8 {
    let mut result = 1;
    let is_blacklisted = configuration::check_ip_blacklist(&ip);
    let current_user_number = client_mem::get_connected_clients();
    let max_clients = configuration::get_max_clients();

    if is_blacklisted == 1 || current_user_number >= max_clients {
        result = 0;
    }

    result
}

fn main() {
    configuration::load_config();
    configuration::show_info_conf();

    let listener = TcpListener::bind(format!("0.0.0.0:{}", configuration::get_port())).unwrap();
    for client in listener.incoming(){
        match client{
            Ok(mut client) => {
                let ip = client_mem::get_client_ip(&mut client);
                println!("New client is connected! (IP: {})", ip);

                thread::spawn(move|| {
                    if is_user_allowed(ip) == 1 {
                        client_mem::add_client(&mut client);
                        handle_client(client);
                    }else{
                        client_mem::shutdown_client(&mut client);
                    }
                });
            },
            Err(e) => {
                println!("Error on client: {}", e);
            }
        }
    }

    drop(listener);
}
