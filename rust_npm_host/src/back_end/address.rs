use std::io::{self, Write};
use std::net::{IpAddr, Ipv4Addr, SocketAddr};

pub fn load_addresses(addresses: &mut Vec<SocketAddr>) {
    println!("Enter IP addresses and sockets to monitor.");
    println!("Format: <IP_ADDRESS>:<SOCKET> (e.g., 192.168.1.1:80)");
    println!("Type 'done' or press Enter on an empty line when finished.");
    

    loop{
        print!("# ");
        io::stdout().flush().expect("Failed to flush stdout");


        let mut input = String::new();

        match io::stdin().read_line(&mut input) {
            Ok(_) => {
                let trimmed_input = input.trim();
                if trimmed_input.is_empty() || trimmed_input.eq_ignore_ascii_case("done") {
                    break;
                }


                let parts: Vec<&str> = trimmed_input.split(':').collect();

                if parts.len() == 2 {
                    let host_str = parts[0].trim();
                    let port_str = parts[1].trim();
                    let default_port: u16 = 443; // Default port if parsing fails or is invalid

                    // Trying to create a IPV4Addr object
                    // Parse the host string into Ipv4Addr
                    match host_str.parse::<Ipv4Addr>() {
                        Ok(parsed_ipv4_addr) => {
                            // IP address parsed successfully.
                            // Now, parse the port string into u16.
                            let port_number = match port_str.parse::<u16>() {
                                Ok(port) => {
                                    if port == 0 { // Port 0 is often reserved or problematic
                                        eprintln!(" -> Warning: Port 0 is generally not usable. Using default port {}.", default_port);
                                        default_port
                                    } else {
                                        port
                                    }
                                }
                                Err(e) => {
                                    eprintln!(" -> Warning: Invalid port number '{}': {}. Using default port {}.", port_str, e, default_port);
                                    default_port // Use default if parsing fails
                                }
                            };

                            let new_socket_addr = SocketAddr::new(IpAddr::V4(parsed_ipv4_addr), port_number);
                            addresses.push(new_socket_addr);
                            println!(" -> Added: {}", new_socket_addr);
                        }
                        Err(e) => {
                            eprintln!(" -> Error: Invalid IP address format '{}': {}", host_str, e);
                        }
                    }
                } else {
                    println!(" -> Invalid format. Please use <IP_ADDRESS>:<PORT> (e.g., 192.168.1.1:80 or google.com:443)");
                }
            }
            Err(error) => {
                eprintln!("Oopsie made an oopsie. A hundred thousand dollar oops made me loopy: {}", error);
                break;
            }
        }
    }
}