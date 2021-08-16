use std::net::{TcpStream, Shutdown};
use std::io::Write;


static mut CLIENT_LIST: Vec<ClientItem> = Vec::new();

struct ClientItem{
    ip: String,
    client: *mut TcpStream
}


pub fn shutdown_client(client: &mut TcpStream){
    println!("Shuting down client connection!");
    let ip = get_client_ip(client);

    remove_client(&ip);
    client.shutdown(Shutdown::Both).unwrap();
}

pub fn add_client(client: &mut TcpStream){
    unsafe{
        let ip = get_client_ip(client);

        CLIENT_LIST.insert(CLIENT_LIST.len(), ClientItem{ ip: ip, client: client });
    }
}

pub fn remove_client(ip: &String){
    unsafe{
        let client = get_client_id(ip);
        CLIENT_LIST.remove(client as usize);
    }
}

pub fn send_message_to_clients(ip: String, data_msg: &[u8]){
    unsafe{
        for current_client in &CLIENT_LIST {
            if current_client.ip != ip {
                println!("Sending Message to ip: {}", current_client.ip);
                (*current_client.client).write_all(data_msg).unwrap();
            }
        }
    }
}

pub fn get_client_ip(client: &mut TcpStream) -> String{
    client.peer_addr().unwrap().ip().to_string()
}


pub fn get_client_id(ip: &String) -> i32 {
    unsafe {
        let mut result:i32 = -1;
        for client in &CLIENT_LIST{
            result+=1;

            if &client.ip == ip {
                break;
            }
        }

        result
    }
}



#[cfg(test)]
mod tests{
    use super::*;

    #[test]
    fn test_add(){
        println!("Done!");
    }
}



