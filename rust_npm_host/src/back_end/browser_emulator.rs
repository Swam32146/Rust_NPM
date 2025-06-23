use thirtyfour::prelude::*;
use std::time::{Duration, Instant};
use tokio;

/// Emulates a web browser to interact with web pages, primarily for measuring load times.
///
/// It uses Selenium WebDriver (via the `thirtyfour` crate) to control a browser instance.
/// A running WebDriver server (e.g., chromedriver, geckodriver) is required.
pub struct BrowserEmulator {
    driver: WebDriver,
}

impl BrowserEmulator {
    /// Creates a new `BrowserEmulator` instance and connects to a WebDriver server.
    ///
    /// # Arguments
    ///
    /// * `webdriver_url`: The URL of the WebDriver server (e.g., "http://localhost:4444").
    /// * `headless`: If `true`, attempts to run the browser in headless mode.
    ///
    /// # Returns
    ///
    /// A `Result` containing the `BrowserEmulator` or a `WebDriverError` if connection fails
    /// or capabilities cannot be set.
    ///
    /// # Notes
    ///
    /// Currently hardcoded to use Chrome. Headless mode arguments are specific to Chrome.
    pub async fn new(webdriver_url: &str, headless: bool) -> Result<Self, WebDriverError> {
        let mut caps = DesiredCapabilities::chrome();
        if headless {
            caps.add_chrome_arg("--headless")?;
            caps.add_chrome_arg("--no-sandbox")?; // Often needed in containerized environments
            caps.add_chrome_arg("--disable-dev-shm-usage")?; // Overcomes limited resource problems
            // You might need more arguments depending on the environment and Chrome version
            // e.g., "--disable-gpu", "--window-size=1920,1080"
        }

        let driver = WebDriver::new(webdriver_url, caps).await?;
        Ok(Self { driver })
    }

    // Default timeout for waiting for an element to become available
    const DEFAULT_ELEMENT_WAIT_TIMEOUT_SECONDS: u64 = 30;

    /// Navigates to the given URL and measures the time until the page is considered "functional".
    ///
    /// "Functional" is defined either by the page load completing or, if `functional_criteria_selector`
    /// is provided, by that specific element being present and visible on the page.
    ///
    /// # Arguments
    ///
    /// * `url`: The URL to navigate to.
    /// * `functional_criteria_selector`: An optional CSS selector. If provided, the function will
    ///   wait for an element matching this selector to be present and visible. The measurement
    ///   stops when this condition is met or a timeout occurs.
    ///
    /// # Returns
    ///
    /// A `Result` containing the `Duration` taken for the page to become functional,
    /// or a `WebDriverError` if navigation fails, the element is not found within the timeout,
    /// or another WebDriver operation fails.
    ///
    /// # Errors
    ///
    /// Can return `WebDriverError::NoSuchElement` or `WebDriverError::Timeout` if `functional_criteria_selector`
    /// is provided and the element is not found or visible within `DEFAULT_ELEMENT_WAIT_TIMEOUT_SECONDS`.
    pub async fn measure_load_time(
        &self,
        url: &str,
        functional_criteria_selector: Option<&str>,
    ) -> Result<Duration, WebDriverError> {
        let start_time = Instant::now();
        self.driver.goto(url).await?;

        if let Some(selector) = functional_criteria_selector {
            let by = By::Css(selector);
            // Wait for the element to be present and visible
            // thirtyfour's find() method implicitly waits for the element to be present.
            // We can also add an explicit wait with a timeout.
            let wait_timeout = Duration::from_secs(DEFAULT_ELEMENT_WAIT_TIMEOUT_SECONDS);
            let mut attempts = 0;
            let max_attempts = wait_timeout.as_secs() * 2; // Check twice per second

            loop {
                match self.driver.query(by.clone()).first().await {
                    Ok(element) => {
                        if element.is_displayed().await? {
                            break; // Element found and is visible
                        }
                    }
                    Err(_) if attempts >= max_attempts => {
                        // Element not found after timeout, return error or handle as needed
                        return Err(WebDriverError::NoSuchElement(format!(
                            "Element with selector '{}' not found or not visible after {} seconds",
                            selector,
                            wait_timeout.as_secs()
                        )));
                    }
                    Err(_) => {
                        // Element not found yet, continue waiting
                    }
                }
                attempts += 1;
                tokio::time::sleep(Duration::from_millis(500)).await;
                if start_time.elapsed() > wait_timeout {
                     return Err(WebDriverError::Timeout(format!(
                        "Timeout waiting for element with selector '{}' after {} seconds",
                        selector,
                        wait_timeout.as_secs()
                    )));
                }
            }
        }
        // If no selector is provided, or after the element is found, the time is recorded.
        let duration = start_time.elapsed();
        Ok(duration)
    }

    /// Closes the browser and quits the WebDriver session.
    ///
    /// This should be called to clean up resources when the emulator is no longer needed.
    ///
    /// # Returns
    ///
    /// A `Result` indicating success or a `WebDriverError` if quitting fails.
    pub async fn close(&self) -> Result<(), WebDriverError> {
        self.driver.quit().await?;
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    // Note: These tests require a running WebDriver server (e.g., chromedriver)
    // and a web server for the target URL if not using an external site.
    // For CI, you'd typically use a mock WebDriver or a controlled test environment.

    // To run chromedriver: `chromedriver --port=4444`
    const WEBDRIVER_URL: &str = "http://localhost:4444";

    #[tokio::test]
    #[ignore] // Ignored because it requires an external WebDriver server
    async fn test_can_launch_browser_and_navigate() {
        // Ensure chromedriver or geckodriver is running on port 4444
        // e.g., chromedriver --port=4444
        let emulator = BrowserEmulator::new(WEBDRIVER_URL, true).await; // Run headless for tests
        assert!(emulator.is_ok(), "Failed to create emulator: {:?}", emulator.err());
        if let Ok(emu) = emulator {
            // Using a known public site for this test.
            // Consider using a local test server for more reliable testing.
            // We'll test without a specific selector first.
            let result_no_selector = emu.measure_load_time("https://www.example.com", None).await;
            assert!(result_no_selector.is_ok(), "Failed to measure load time (no selector): {:?}", result_no_selector.err());
            if let Ok(duration) = result_no_selector {
                println!("Load time for example.com (no selector): {:?}", duration);
                assert!(duration.as_millis() > 0);
            }

            // Test with a selector that should exist on example.com (e.g., the `<h1>` tag)
            let result_with_selector = emu.measure_load_time("https://www.example.com", Some("h1")).await;
            assert!(result_with_selector.is_ok(), "Failed to measure load time (with selector 'h1'): {:?}", result_with_selector.err());
            if let Ok(duration) = result_with_selector {
                println!("Load time for example.com (with selector 'h1'): {:?}", duration);
                assert!(duration.as_millis() > 0);
            }
            assert!(emu.close().await.is_ok());
        }
    }

    #[tokio::test]
    #[ignore] // Ignored because it requires an external WebDriver server
    async fn test_new_emulator_error_if_webdriver_not_running() {
        let emulator = BrowserEmulator::new("http://localhost:9999", true).await; // Assume nothing running on 9999
        assert!(emulator.is_err(), "Should have failed to create emulator if WebDriver is not running");
    }

    #[tokio::test]
    #[ignore] // Ignored because it requires an external WebDriver server
    async fn test_measure_load_time_element_timeout() {
        let emulator = BrowserEmulator::new(WEBDRIVER_URL, true).await;
        assert!(emulator.is_ok(), "Failed to create emulator: {:?}", emulator.err());
        if let Ok(emu) = emulator {
            let result = emu.measure_load_time("https://www.example.com", Some("#nonexistent-element-for-timeout-test-123987")).await;
            assert!(result.is_err(), "Expected timeout error when element does not exist");
            if let Err(e) = result {
                let error_string = format!("{:?}", e);
                // Check for NoSuchElement or Timeout, as behavior might vary slightly or be wrapped
                assert!(
                    error_string.contains("NoSuchElement") || error_string.contains("Timeout"),
                    "Unexpected error type for non-existent element: {}", error_string
                );
            }
            assert!(emu.close().await.is_ok());
        }
    }
}
