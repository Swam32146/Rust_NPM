use std::io::{self, Write};
use std::net::{IpAddr, Ipv4Addr, SocketAddr};

pub fn load_addresses(addresses: &mut Vec<SocketAddr>) {
    println!("Enter IP addresses and sockets to monitor.");
    println!("Format: <IP_ADDRESS>:<SOCKET> (e.g., 192.168.1.1:80)");
    println!("Type 'done' or press Enter on an empty line when finished.");

    loop {
        print!("# ");
        io::stdout().flush().expect("Failed to flush stdout");

        let mut input = String::new();
        match io::stdin().read_line(&mut input) {
            Ok(_) => {
                let trimmed_input = input.trim();
                if trimmed_input.is_empty() || trimmed_input.eq_ignore_ascii_case("done") {
                    break;
                }

                if let Some(new_addr) = parse_socket_address(trimmed_input) {
                    addresses.push(new_addr);
                    println!(" -> Added: {}", new_addr);
                }
            }
            Err(error) => {
                eprintln!("Oopsie made an oopsie: {}", error);
                break;
            }
        }
    }
}

pub fn add_new_address(addresses: &mut Vec<SocketAddr>) {
    println!("Enter the new IP address and port to add.");
    print!("# ");
    io::stdout().flush().expect("Failed to flush stdout");

    let mut input = String::new();
    if io::stdin().read_line(&mut input).is_ok() {
        let trimmed_input = input.trim();
        if !trimmed_input.is_empty() {
            if let Some(new_addr) = parse_socket_address(trimmed_input) {
                addresses.push(new_addr);
                println!(" -> Added: {}", new_addr);
            }
        }
    }
}

fn parse_socket_address(input: &str) -> Option<SocketAddr> {
    let parts: Vec<&str> = input.split(':').collect();

    if parts.len() == 2 {
        let host_str = parts[0].trim();
        let port_str = parts[1].trim();
        let default_port: u16 = 443;

        match host_str.parse::<Ipv4Addr>() {
            Ok(parsed_ipv4_addr) => {
                let port_number = match port_str.parse::<u16>() {
                    Ok(port) if port > 0 => port,
                    _ => {
                        eprintln!(" -> Warning: Invalid port number. Using default port {}.", default_port);
                        default_port
                    }
                };
                Some(SocketAddr::new(IpAddr::V4(parsed_ipv4_addr), port_number))
            }
            Err(e) => {
                eprintln!(" -> Error: Invalid IP address format '{}': {}", host_str, e);
                None
            }
        }
    } else {
        println!(" -> Invalid format. Please use <IP_ADDRESS>:<PORT>");
        None
    }
}