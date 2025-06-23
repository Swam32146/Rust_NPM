use std::net::{IpAddr, Ipv4Addr, SocketAddr};
use std::time::Duration;
use std::thread;
mod back_end;
mod front_end;
// Need to import the function if we're calling it directly here
// use back_end::ping_test::measure_website_functional_time;

// Example function demonstrating how to call the website check
// In a real scenario, parameters would come from GUI input or config files.
async fn perform_website_check(
    webdriver_url: &str,
    target_url: &str,
    selector: Option<&str>,
    headless: bool,
) {
    println!(
        "Performing website check for {} (headless: {})",
        target_url, headless
    );
    match back_end::ping_test::measure_website_functional_time(
        webdriver_url,
        target_url,
        selector,
        headless,
    )
    .await
    {
        Ok(duration) => {
            println!(
                "Website {} functional in {:?}{}",
                target_url,
                duration,
                selector.map_or("".to_string(), |s| format!(" (waiting for '{}')", s))
            );
        }
        Err(e) => {
            eprintln!("Error checking website {}: {:?}", target_url, e);
        }
    }
}

#[tokio::main]
async fn main() {
    //I am trying the gui interface rn
    // front_end::application::run_gui(); // This would also need to be async or spawn tasks

    // Example usage of the website check.
    // In a real app, these values would come from user input, config, etc.
    // And this call would likely be triggered by a UI event.
    perform_website_check(
        "http://localhost:4444",      // WebDriver URL
        "https://www.example.com",    // Target website
        Some("h1"),                   // Optional: CSS selector for "functional"
        true,                         // Run headless
    )
    .await;

    perform_website_check(
        "http://localhost:4444",
        "https://www.example.com",
        None, // No specific element, just page load
        false, // Run with a visible browser (if not overridden by WebDriver default)
    )
    .await;

    // If you still want to run the GUI, it needs to be compatible with the async main.
    // For example, if run_gui() itself can be async:
    // front_end::application::run_gui().await;
    // Or if it's synchronous but needs to run within the tokio runtime:
    // tokio::task::spawn_blocking(front_end::application::run_gui).await.unwrap();
    // For now, I'll keep the original call commented out as its integration
    // with async is beyond the current scope. The perform_website_check calls above
    // demonstrate the core functionality.

    println!("GUI part would run here. For now, example checks are complete.");


    // front_end::application::run_gui();

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
