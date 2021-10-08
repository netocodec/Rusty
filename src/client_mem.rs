use std::net::{TcpStream, Shutdown, SocketAddr, UdpSocket};
use std::io::Write;
use std::ptr;

/// This is the memory sector of the client list.
static mut CLIENT_LIST: Vec<ClientItem> = Vec::new();

/// Structure of the Client.
#[derive(Debug)]
struct ClientItem{
    /// Ip of the client
    ip: String,
    /// Socket Origin
    net_origin: *mut SocketAddr,
    /// Client Object
    client: *mut TcpStream,
    /// UDP Client Object
    udp_client: *mut UdpSocket
}


/// This will disconnect the client from the connection with the server.
pub fn shutdown_client(client: &mut TcpStream){
    println!("Shuting down client connection!");
    let ip = get_client_ip(client);

    remove_client(&ip);
    client.shutdown(Shutdown::Both).unwrap();
}

/// This will add the client into the memory vector.
pub fn add_client(client: &mut TcpStream){
    unsafe{
        let ip = get_client_ip(client);

        CLIENT_LIST.insert(CLIENT_LIST.len(), ClientItem{ ip: ip, client: client, net_origin: ptr::null_mut(), udp_client: ptr::null_mut() });
    }
}

/// This will add the udp client into the memory vector
pub fn add_udp_client(client: &mut UdpSocket, src: &mut SocketAddr){
    unsafe {
        let ip = get_udp_client_ip(src);

        if get_client_id(&ip) == -1 {
            CLIENT_LIST.insert(CLIENT_LIST.len(), ClientItem{ ip: ip, client: ptr::null_mut(), net_origin: src, udp_client: client });

        }
    }
}

/// This will remove the client if the ip is found on the list.
pub fn remove_client(ip: &String){
    unsafe{
        let client = get_client_id(ip);

        if client != -1 {
            CLIENT_LIST.remove(client as usize);
        }
    }
}

/// This function will send the message to all the clients.
pub fn send_message_to_clients(ip: String, data_msg: &[u8]){
    unsafe{
        for current_client in &CLIENT_LIST {
            if current_client.ip != ip {
                println!("Sending Peer Message: {} --> {}", ip, current_client.ip);
                if !current_client.client.is_null() {
                    (*current_client.client).write_all(data_msg).unwrap();
                }else{
                    (*current_client.udp_client).send_to(data_msg, &current_client.ip).unwrap();
                }
            }
        }
    }
}

/// Gets the client IP.
pub fn get_client_ip(client: &mut TcpStream) -> String{
    client.peer_addr().unwrap().ip().to_string()
}

/// Gets the udp client IP.
pub fn get_udp_client_ip(client: &mut SocketAddr) -> String{
    format!("{}:{}", client.ip().to_string(), client.port().to_string())
}


/// This searchs for the client IP and return the client index.
pub fn get_client_id(ip: &String) -> i32 {
    unsafe {
        let mut result:i32 = -1;
        let mut c:i32 = -1;

        for client in &CLIENT_LIST{
            c+=1;

            if &client.ip == ip {
                result = c;
                break;
            }
        }

        result
    }
}

/// Gets the number of clients on the memory list.
pub fn get_connected_clients() -> usize{
    unsafe{
        CLIENT_LIST.len()
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



