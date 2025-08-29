use std::net::{IpAddr, Ipv4Addr, SocketAddr};
use std::time::Duration;
use tokio::io::{self, AsyncBufReadExt, BufReader};
use tokio::time;
mod back_end;
mod front_end;

#[tokio::main]
async fn main() {

    println!("GUI part would run here. For now, example checks are complete.");

    let timeout: u64 = 6;


    // front_end::application::run_gui();

    // Define timeout once as it's constant for all pings.
    let ping_timeout: Duration = Duration::from_secs(4);

    // Here I am going to add a funciton that loads all the ip addresses that I need.
    let mut addresses: Vec<SocketAddr> = Vec::new();
    back_end::address::load_addresses(&mut addresses);

    // Setup asynchronous stdin reader
    let mut stdin = BufReader::new(io::stdin());
    let mut line = String::new();

    loop {

        println!("Results for listed addresses:")
        for address_entry in &addresses {
            let print_addr: String = address_entry.to_string();
            // Store the result to avoid calling is_port_open twice
            let is_open = back_end::ping_test::is_port_open(*address_entry, ping_timeout);

            if is_open {
                println!("{} Is Open :)", print_addr);
            } else {
                println!("{} Is Closed :(", print_addr);
            }
        }

        tokio::select! {
            _ = time::sleep(Duration::from_secs(timeout)) => {
                // Once timeout passed, continue the loop
            }
            result = stdin.read_line(&mut line) => {
                match result {
                    Ok(_) => {
                        let trimmed_line = line.trim().to_lowercase();
                        if trimmed_line == "exit" || trimmed_line.is_empty() {
                            println!("We gon, hasta la vista baby");
                            break;
                        } else {
                            println!("What did you put in, idk what to do here. \nheck is this? '{}' \night lets keep it pushing", trimmed_line);
                            line.clear();
                        }
                    }
                    Err(e) => {
                        eprintln!("Error reading line: {}", e);
                        break;
                    }
                }
            }
        }
    }
    println!("Program finished.");
}
