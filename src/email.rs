pub mod email_functions {
  extern crate imap;
  extern crate native_tls;

  use std::env;
  use dotenv::dotenv;
  use scraper::{
    Html,
    Selector
  };

  fn extract_confirmation_number(html_content: &str) -> Option<String> {
      let fragment = Html::parse_fragment(html_content);
      let selector = Selector::parse("td[data-nc*=code]").unwrap(); // Adjust the selector as needed

      for td in fragment.select(&selector) {
        let text = td.text().collect::<String>();
        let cleaned_text: String = text.chars().filter(|c| c.is_ascii_alphanumeric()).collect();
        return Some(cleaned_text);
      }

      None
  }

  pub fn get_confirmation_email() -> imap::error::Result<Option<String>> {
      dotenv().ok();
      let username = env::var("GMAIL_USERNAME").expect("GMAIL_USERNAME not found in the .env file");
      let password = env::var("GMAIL_PASSWORD").expect("GMAIL_PASSWORD not found in the .env file");

      // Connect to the IMAP server
      let domain = "imap.gmail.com";

      let tls = native_tls::TlsConnector::builder().build().unwrap();

      // we pass in the domain twice to check that the server's TLS
      // certificate is valid for the domain we're connecting to.
      let client = imap::connect((domain, 993), domain, &tls).unwrap();

      // the client we have here is unauthenticated.
      // to do anything useful with the e-mails, we need to log in
      let mut imap_session = client
          .login(username, password)
          .map_err(|e| e.0)?;

      // We want to fetch emails with the subject "Your confirmation code".
      let query = "SUBJECT \"Your confirmation code\"";
      let mailbox = "INBOX";
      imap_session.select(mailbox)?;

      let messages = imap_session.search(query)?;

      // Find the UID of the newest message
      let newest_message = messages.iter().max();

      // Fetch the newest message, if any
      let message_uid = if let Some(uid) = newest_message {
          uid
      } else {
          println!("No email found with the subject \"Your confirmation code\"");
          imap_session.logout()?;
          return Ok(None);
      };

      // Fetch the body of the email.
      let messages = imap_session.fetch(format!("{}", message_uid), "RFC822")?;
      let message = if let Some(m) = messages.iter().next() {
          m
      } else {
          println!("Failed to fetch the email body");
          imap_session.logout()?;
          return Ok(None);
      };

      // Extract the message's body.
      let body = if let Some(body) = message.body() {
          String::from_utf8_lossy(body)
      } else {
          println!("The email body is empty");
          imap_session.logout()?;
          return Ok(None);
      };

      // parse the body for the confirmation code
      let confirmation_code = self::extract_confirmation_number(&body);
      match confirmation_code {
        Some(v) => {
          // Be nice to the server and log out.
          imap_session.logout()?;

          Ok(Some(v)) 
        }
        None => {
          println!("Could not find a confirmation code in the email.");

          // Be nice to the server and log out.
          imap_session.logout()?;

          Ok(None)
        }
      }
  }
}