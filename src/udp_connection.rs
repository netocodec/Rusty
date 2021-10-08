use std::net::UdpSocket;
use std::io::Read;


#[path = "client_mem.rs"]
mod client_mem;
#[path = "configuration.rs"]
mod configuration;



/// Starts the connection of the UDP server.
pub async fn start_connection(){
    let mut socket = UdpSocket::bind(format!("0.0.0.0:{}", configuration::get_port())).unwrap();
    socket.set_broadcast(true).expect("set_broadcast call failed!");

    loop {
        let mut buf = [0 as u8; 50];
        let (amt, mut src) = socket.recv_from(&mut buf).unwrap();

        if amt > 1{
            let ip = client_mem::get_udp_client_ip(&mut src);
            println!("New client is connected! (IP: {})", ip);

            if configuration::is_user_allowed(ip) == 1{
                let ip = client_mem::get_udp_client_ip(&mut src);
                client_mem::add_udp_client(&mut socket, &mut src);

                let buf = &mut buf[..amt];
                client_mem::send_message_to_clients(ip, &buf[..amt]);
            }
        }
    }
}

