use std::net::{TcpListener, TcpStream};
use std::io::Read;
use std::thread;
mod client_mem;


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

fn main() {
    let listener = TcpListener::bind("0.0.0.0:7005").unwrap();

    for client in listener.incoming(){
        match client{
            Ok(mut client) => {
                println!("New client is connected!");

                thread::spawn(move|| {
                    client_mem::add_client(&mut client);
                    handle_client(client);
                });
            },
            Err(e) => {
                println!("Error on client: {}", e);
            }
        }
    }

    drop(listener);
}
