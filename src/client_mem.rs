use std::net::{TcpStream, Shutdown, IpAddr, Ipv4Addr, SocketAddr};
use std::collections::HashSet;

static mut client_list: Vec<ClientItem> = Vec::new();

struct ClientItem{
    ip: String,
    client: *mut TcpStream
}

type ClientItemResult = Result<u8, ClientItem>;


pub fn shutdown_client(client: &mut TcpStream){
    println!("Shuting down client connection!");
    let ip = get_client_ip(client);

    remove_client(&ip);
    client.shutdown(Shutdown::Both).unwrap();
}

pub fn add_client(client: &mut TcpStream){
    unsafe{
        let ip = get_client_ip(client).unwrap();

        client_list.insert(client_list.len(), ClientItem{ ip: ip, client: client });
    }
}

pub fn remove_client(ip: &String){
    unsafe{
        let client = get_client(ip);
        client_list.remove(ip);
    }
}

pub fn send_message_to_clients(ip: String, _data_msg: &[u8]){
    unsafe{
        for client in &client_list {
            if client.ip != ip {
                println!("IP: {} | FOR IP: {} | Client: {:?}", ip, key, value);
                //value.write(data_msg).unwrap();
            }
        }
    }
}

pub fn get_client_ip(client: &mut TcpStream) -> String{
    client.local_addr().unwrap().ip().to_string()
}


pub fn get_client(ip: &String) -> ClientItemResult {
    unsafe {
        let mut result = 0;
        for client in &client_list{
            if client.ip == ip {
                result = 1;
                Ok(client);
                break;
            }
        }

        if result == 0{
            Err(-1);
        }
    }
}



#[cfg(test)]
mod tests{
    use super::*;

    #[test]
    fn test_add(){
        let test_ip_str: String = "127.0.0.1:7005".to_string();
        let test_ip: SocketAddr = SocketAddr::new(IpAddr::V4(Ipv4Addr::new(127, 0, 0, 1)), 7005);

        match TcpStream::connect(test_ip){
            Ok(stream) => {
                println!("Conected on server!");
                add_client(stream, test_ip_str);
            },
            Err(e)=>{
                println!("Error: {}", e);
            }
        }

    }
}



