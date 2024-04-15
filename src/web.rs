pub mod web_functions {
  use tokio;
  use dotenv::dotenv;
  use thirtyfour::prelude::*;
  use crate::email::email_functions;
  use reqwest::{
    get,
    Error
  };
  use std::{
      env, 
      thread::sleep,
      time::Duration
  };

  #[tokio::main]
  pub async fn get_ip_address() -> Result<String, Error> {
    // check if home ip has changed
    match get("https://api.ipify.org").await {
      Ok(response) => {
        match response.text().await {
          Ok(ip) => {
            Ok(ip)
          }
          Err(e) => {
            Err(e)
          }
        }
      }
      Err(e) => {
        Err(e)
      }
    }
  }

  #[tokio::main]
  pub async fn web_automation() -> WebDriverResult<()> {
      dotenv().ok();
      let username = env::var("NAMECHEAP_USERNAME").expect("NAMECHEAP_USERNAME not found in the .env file");
      let password = env::var("NAMECHEAP_PASSWORD").expect("NAMECHEAP_PASSWORD not found in the .env file");

      let caps = DesiredCapabilities::chrome();
      let driver = WebDriver::new("http://localhost:9515", caps).await?;

      driver.goto("https://namecheap.com").await?;

      driver.maximize_window().await?;

      // plenty of time to pop the browser and maximize
      sleep(Duration::from_secs(5));

      let account_button = driver.query(By::XPath("/html/body/fragment[1]/layout-header/div/header/div/div[2]/div/nav/div/div[9]/div[1]/span")).first().await?;
      account_button.click().await?;

      let username_input = driver.query(By::XPath("//*[@id=\"ctl00_ctl00_ctl00_ctl00_base_content_web_base_content_home_content_page_content_left_ctl02_loginDiv\"]/ul/li/fieldset/div[2]/input")).first().await?;
      username_input.send_keys(username).await?;

      let password_input = driver.query(By::XPath("//*[@id=\"ctl00_ctl00_ctl00_ctl00_base_content_web_base_content_home_content_page_content_left_ctl02_loginDiv\"]/ul/li/fieldset/div[3]/input")).first().await?;
      password_input.send_keys(password).await?;

      let sign_in_button = driver.query(By::XPath("//*[@id=\"ctl00_ctl00_ctl00_ctl00_base_content_web_base_content_home_content_page_content_left_ctl02_LoginButton\"]")).first().await?;
      sign_in_button.click().await?;

      // allow time for confirmation email to come through
      sleep(Duration::from_secs(5));

      // get confirmation code from email
      let result = email_functions::get_confirmation_email();

      match result {
          Ok(Some(code)) => {
              let verification_input = driver.query(By::XPath("//*[@id=\"codeInput\"]")).first().await?;
              verification_input.send_keys(code).await?;

              let verification_submit = driver.query(By::Css("button.gb-btn.gb-btn--primary.gb-btn--block-sm.gb-mb-2[data-ncid=continue]")).first().await?;
              verification_submit.click().await?;

              let manage_button = driver.query(By::Css("a.btn.btn-white.btn-small-uppercase[data-ng-if=\"(actionList[0].DisplayText | lowercase) !='learn more'\"]")).first().await?;
              manage_button.click().await?;

              let dns_tab_button = driver.query(By::Css("a[href=\"/Domains/DomainControlPanel/jonathanshreckengost.com/advancedns\"]")).first().await?;
              dns_tab_button.click().await?;
          }
          Ok(None) => {
              print!("No confirmation code found in the email.");
          }
          Err(error) => {
              print!("An error occurred: {:?}", error);
          }
      }

      // Always explicitly close the browser.
      driver.quit().await?;

      Ok(())
  }
}