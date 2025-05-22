use std::time::Duration;
use std::net::{SocketAddr, TcpStream};

pub fn is_port_open(addr: SocketAddr, timeout: Duration) -> bool {
    let connection_result = TcpStream::connect_timeout(&addr, timeout).is_ok();
    return connection_result;
}

