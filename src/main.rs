use std::net::{TcpListener, TcpStream};
use std::io::Read;
use std::thread;

mod configuration;
mod udp_connection;
mod tcp_connection;


#[tokio::main]
async fn main() {
    configuration::load_config();
    configuration::show_info_conf();

    if configuration::get_connection_type() == configuration::ConnectionType::TCP {
        tcp_connection::start_connection().await;
    }else{
        udp_connection::start_connection().await;    
    }
}



