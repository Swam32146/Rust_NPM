use std::net::{IpAddr, Ipv4Addr, SocketAddr};
use std::time::Duration;
use std::thread;
mod back_end;
mod front_end;

fn main() {


    //I am trying the gui interface rn

    front_end::application::run_gui();

    /*
    // let addr = SocketAddr::new(IpAddr::V4(Ipv4Addr::new(8, 8, 8, 8)), 53);
    let timeout: Duration = Duration::from_secs(1);

    // Here I am going to add a funciton that loads all the ip addresses that I need.

    let mut addresses: Vec<SocketAddr> = Vec::new();
    back_end::address::load_addresses(&mut addresses);

    loop {
        for address_entry in &addresses {
            let print_addr: String = address_entry.to_string();
            println!("{}", back_end::ping_test::is_port_open(*address_entry, timeout));
            

            if back_end::ping_test::is_port_open(*address_entry, timeout) {
                println!("{} Is Open : )", print_addr);
            } else {
                println!("{} Is Closed : (", print_addr);
            }

        }





        let sleep_duration = Duration::from_secs(60);
        thread::sleep(sleep_duration);

        

    }

    */
}
