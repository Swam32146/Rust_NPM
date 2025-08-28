use std::time::Duration;
use std::net::{SocketAddr, TcpStream};

pub fn is_port_open(addr: SocketAddr, timeout: Duration) -> bool {
    TcpStream::connect_timeout(&addr, timeout).is_ok()
}


