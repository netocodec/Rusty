use std::net::UdpSocket;
use std::io::Read;


#[path = "client_mem.rs"]
mod client_mem;
#[path = "configuration.rs"]
mod configuration;


fn handle_client(socket: &mut UdpSocket){
    let mut buf = [0; 50];
    let (amt, mut src) = socket.recv_from(&mut buf).unwrap();
    let ip = client_mem::get_udp_client_ip(&mut src);

    println!("New client is connected! (IP: {})", ip);

    let buf = &mut buf[..amt];
    socket.send_to(buf, &src).unwrap();
}


/// Starts the connection of the UDP server.
pub fn start_connection(){
    let mut socket = UdpSocket::bind(format!("0.0.0.0:{}", configuration::get_port())).unwrap();
    loop {
        let mut buf = [0; 50];
        let (_, mut src) = socket.recv_from(&mut buf).unwrap();
        let ip = client_mem::get_udp_client_ip(&mut src);
        println!("New client is connected! (IP: {})", ip);

        if configuration::is_user_allowed(ip) == 1{
            handle_client(&mut socket);
        }
    }
}

