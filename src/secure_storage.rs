use keyring::{Entry};
use std::error::Error;

pub fn get_api_key() -> Result<String, Box<dyn Error>> {
  let entry = Entry::new("ask_cli", "open_ai")?;

  let password = match entry.get_password() {
      Ok(password) => password,
      Err(_) => {
          println!("Please enter your OpenAI API key:");
          let mut key = String::new();
          std::io::stdin().read_line(&mut key)?;

          key = key.trim().to_string();

          entry.set_password(&key).unwrap_or_else(|err| {
            // TODO: fix this shit on linux
            println!("An error occurred: {}", err);
            ()
          });

          key
      }
  };

  Ok(password)
}