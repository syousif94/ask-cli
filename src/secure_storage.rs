use keyring::{Entry};
use std::error::Error;
use std::fs;
use std::path::PathBuf;

pub fn get_api_key() -> Result<String, Box<dyn Error>> {
  let entry = Entry::new("ask_cli", "open_ai")?;

  let password = match entry.get_password() {
      Ok(password) => password,
      Err(_) => {
          let env_path = get_env_path()?;
          if env_path.exists() {
              let key = fs::read_to_string(&env_path)?;
              key.trim().to_string()
          } else {
              println!("Please enter your OpenAI API key:");
              let mut key = String::new();
              std::io::stdin().read_line(&mut key)?;
              key = key.trim().to_string();

              entry.set_password(&key).unwrap_or_else(|_err| {
                fs::write(env_path, &key).unwrap_or_else(|_err| {
                  println!("Failed to write key to file");
                });
              });
              
              key
          }
      }
  };

  Ok(password)
}

fn get_env_path() -> Result<PathBuf, Box<dyn Error>> {
    let home_dir = dirs::home_dir().ok_or("Failed to get home directory")?;
    let dir_path = home_dir.join(".ask-cli");
    fs::create_dir_all(&dir_path)?;
    let file_path = dir_path.join(".env");
    Ok(file_path)
}