use std::io::{self, Write};
use std::net::SocketAddr;
use crate::back_end;
use std::time::Duration;

pub fn run_cli(addresses: &mut Vec<SocketAddr>) {
    loop {
        print_menu();
        let choice = get_user_choice();

        match choice {
            Some(1) => show_addresses(addresses),
            Some(2) => back_end::address::add_new_address(addresses),
            Some(3) => remove_address(addresses),
            Some(4) => test_all_addresses(addresses),
            Some(5) => {
                println!("Exiting.");
                break;
            }
            _ => println!("Invalid choice, please try again."),
        }
    }
}

fn print_menu() {
    println!("\n--- Network Monitor CLI ---");
    println!("1. Show current IP/port list");
    println!("2. Add IP/port");
    println!("3. Remove IP/port");
    println!("4. Test IP/port");
    println!("5. Exit");
    print!("> ");
    io::stdout().flush().unwrap();
}

fn get_user_choice() -> Option<u32> {
    let mut input = String::new();
    io::stdin().read_line(&mut input).ok()?;
    input.trim().parse::<u32>().ok()
}

fn show_addresses(addresses: &Vec<SocketAddr>) {
    if addresses.is_empty() {
        println!("The list of addresses is empty.");
        return;
    }
    println!("Current IP/port list:");
    for (i, addr) in addresses.iter().enumerate() {
        println!("{}. {}", i + 1, addr);
    }
}

fn remove_address(addresses: &mut Vec<SocketAddr>) {
    if addresses.is_empty() {
        println!("Address list is already empty.");
        return;
    }

    show_addresses(addresses);
    println!("Enter the number of the address to remove:");
    print!("> ");
    io::stdout().flush().unwrap();

    let mut input = String::new();
    if io::stdin().read_line(&mut input).is_ok() {
        if let Ok(num) = input.trim().parse::<usize>() {
            if num > 0 && num <= addresses.len() {
                let removed = addresses.remove(num - 1);
                println!("Removed: {}", removed);
            } else {
                println!("Invalid number.");
            }
        } else {
            println!("Invalid input.");
        }
    }
}

fn test_all_addresses(addresses: &Vec<SocketAddr>) {
    let ping_timeout: Duration = Duration::from_secs(4);
    println!("Testing all addresses...");
    if addresses.is_empty() {
        println!("The list of addresses is empty.");
        return;
    }
    for address_entry in addresses {
        let print_addr: String = address_entry.to_string();
        let is_open = back_end::ping_test::is_port_open(*address_entry, ping_timeout);

        if is_open {
            println!("{} Is Open :)", print_addr);
        } else {
            println!("{} Is Closed :(", print_addr);
        }
    }
}