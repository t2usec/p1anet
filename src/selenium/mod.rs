use std::time::Duration;

use thirtyfour::{By, ChromiumLikeCapabilities, Cookie, DesiredCapabilities, WebDriver};

pub async fn get_ctfshow_cookies() -> Result<Vec<Cookie>, anyhow::Error> {
    let mut caps = DesiredCapabilities::chrome();
    caps.add_arg("headless")?;
    caps.add_arg("no-sandbox")?;
    caps.add_arg("window-size=1920,1080")?;

    let driver = WebDriver::new(crate::config::get_config().selenium.uri.clone(), caps).await?;

    driver.get("https://ctf.show/login").await?;

    // Find the username and password fields, then send keys
    driver
        .find(By::Id("name"))
        .await?
        .send_keys(crate::config::get_config().ctfshow.name.clone())
        .await?;
    driver
        .find(By::Id("password"))
        .await?
        .send_keys(crate::config::get_config().ctfshow.password.clone())
        .await?;

    // Find the submit button and click it
    let submit_btn = driver.find(By::Id("_submit")).await?;
    submit_btn.click().await?;

    // Wait for a short duration to ensure the login is processed
    tokio::time::sleep(Duration::from_secs(2)).await;

    // Get cookies from the session
    let cookies = driver.get_all_cookies().await?;

    // Close the browser session
    driver.quit().await?;

    return Ok(cookies);
}
