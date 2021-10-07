use std::net::{TcpListener, TcpStream};
use std::io::Read;
use std::thread;


#[path = "client_mem.rs"]
mod client_mem;
#[path = "configuration.rs"]
mod configuration;


/// Handle funnction of the client connection.
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



pub fn start_connection(){
    let listener = TcpListener::bind(format!("0.0.0.0:{}", configuration::get_port())).unwrap();
    for client in listener.incoming(){
        match client{
            Ok(mut client) => {
                let ip = client_mem::get_client_ip(&mut client);
                println!("New client is connected! (IP: {})", ip);

                thread::spawn(move|| {
                    if configuration::is_user_allowed(ip) == 1 {
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

