use std::net::SocketAddr;
mod back_end;
mod front_end;
mod cli;

#[tokio::main]
async fn main() {
    println!("Starting Network Monitor");

    let mut addresses: Vec<SocketAddr> = Vec::new();
    back_end::address::load_addresses(&mut addresses);

    cli::run_cli(&mut addresses);

    println!("Program finished.");
}