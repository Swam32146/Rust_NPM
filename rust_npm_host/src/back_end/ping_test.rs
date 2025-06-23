use std::time::Duration;
use std::net::{SocketAddr, TcpStream};
use thirtyfour::WebDriverError; // Added for error type

use super::browser_emulator::BrowserEmulator; // Import BrowserEmulator

pub fn is_port_open(addr: SocketAddr, timeout: Duration) -> bool {
    TcpStream::connect_timeout(&addr, timeout).is_ok()
}

/// Measures the time it takes for a website to become "functional" by emulating a browser.
///
/// This function initializes a `BrowserEmulator`, navigates to the target URL, and waits
/// until specified criteria (e.g., an element appearing) are met, or the page fully loads.
///
/// # Arguments
///
/// * `webdriver_url`: URL of the Selenium WebDriver server (e.g., "http://localhost:4444").
/// * `target_url`: The URL of the website to measure.
/// * `functional_criteria_selector`: Optional CSS selector for an element that, when visible,
///   indicates the site is functional. If `None`, measures up to page load.
/// * `headless`: If `true`, attempts to run the browser in headless mode.
///
/// # Returns
///
/// A `Result` containing the `Duration` taken for the site to become functional, or a
/// `Box<dyn std::error::Error>` if any part of the process fails (e.g., WebDriver connection,
/// navigation, element timeout).
///
/// # Prerequisites
///
/// * A running Selenium WebDriver server (chromedriver or geckodriver) accessible at `webdriver_url`.
/// * The browser controlled by WebDriver (e.g., Chrome) must be installed on the system
///   where WebDriver is running.
pub async fn measure_website_functional_time(
    webdriver_url: &str,
    target_url: &str,
    functional_criteria_selector: Option<&str>,
    headless: bool, // Added headless option
) -> Result<Duration, Box<dyn std::error::Error>> {
    let emulator = BrowserEmulator::new(webdriver_url, headless).await?;

    let load_time = emulator
        .measure_load_time(target_url, functional_criteria_selector)
        .await;

    // Ensure the browser is closed even if measure_load_time fails
    let close_result = emulator.close().await;

    match load_time {
        Ok(duration) => {
            if let Err(e) = close_result {
                // Log or handle browser close error, but prioritize returning load time
                eprintln!("Error closing browser: {:?}", e);
            }
            Ok(duration)
        }
        Err(e) => {
            // If measure_load_time failed, try to close browser and return the original error
            if let Err(close_e) = close_result {
                eprintln!("Error closing browser after load error: {:?}", close_e);
                // Potentially wrap both errors or prioritize the load error
            }
            Err(Box::new(e)) // Convert WebDriverError to a trait object
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    // Basic test for is_port_open - requires a listening port (e.g., a simple netcat listener)
    // `nc -l 8080`
    #[test]
    #[ignore] // Requires a local listener
    fn test_is_port_open_true() {
        use std::net::{IpAddr, Ipv4Addr};
        let addr = SocketAddr::new(IpAddr::V4(Ipv4Addr::new(127, 0, 0, 1)), 8080);
        assert!(is_port_open(addr, Duration::from_secs(1)));
    }

    #[test]
    fn test_is_port_open_false() {
        use std::net::{IpAddr, Ipv4Addr};
        let addr = SocketAddr::new(IpAddr::V4(Ipv4Addr::new(127, 0, 0, 1)), 12345); // Assume port 12345 is not open
        assert!(!is_port_open(addr, Duration::from_secs(1)));
    }

    // Integration test for measure_website_functional_time
    // Requires a running WebDriver (e.g., `chromedriver --port=4444`)
    const WEBDRIVER_URL_TEST: &str = "http://localhost:4444";

    #[tokio::test]
    #[ignore] // Requires WebDriver and internet or a local web server
    async fn test_measure_website_functional_time_example_com() {
        let result = measure_website_functional_time(
            WEBDRIVER_URL_TEST,
            "https://www.example.com",
            Some("h1"), // example.com has an H1
            true,       // headless
        )
        .await;

        assert!(result.is_ok(), "measure_website_functional_time failed: {:?}", result.err());
        if let Ok(duration) = result {
            println!("Functional time for example.com (waiting for H1): {:?}", duration);
            assert!(duration.as_millis() > 0);
        }
    }

    #[tokio::test]
    #[ignore] // Requires WebDriver
    async fn test_measure_website_functional_time_nonexistent_element() {
        let result = measure_website_functional_time(
            WEBDRIVER_URL_TEST,
            "https://www.example.com",
            Some("#nonexistent-element-id-12345"),
            true, // headless
        )
        .await;

        assert!(result.is_err(), "Expected an error when element is not found, but got Ok: {:?}", result.ok());
        if let Err(e) = result {
            // Check if the error is a WebDriverError::NoSuchElement or WebDriverError::Timeout
            // The exact error type might be wrapped in a Box, so string matching might be easier here for a test
            let error_string = format!("{:?}", e);
            assert!(
                error_string.contains("NoSuchElement") || error_string.contains("Timeout"),
                "Unexpected error type: {}", error_string
            );
        }
    }
     #[tokio::test]
    #[ignore] // Requires WebDriver
    async fn test_measure_website_functional_time_invalid_webdriver_url() {
        let result = measure_website_functional_time(
            "http://localhost:9999", // Assuming nothing is running on port 9999
            "https://www.example.com",
            None,
            true, // headless
        )
        .await;

        assert!(result.is_err());
        // We could check for a specific connection error if desired
        // e.g., format!("{:?}", result.err().unwrap()).contains("error sending request")
    }
}

