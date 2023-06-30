use keyring::{Entry};
use std::error::Error;
use std::fs;
use std::path::PathBuf;

const ENV_FILE: &str = ".ask-cli/.env";

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

              entry.set_password(&key).unwrap_or_else(|err| {
                write_error_to_file(&err)?;
              });

              fs::write(env_path, &key)?;
              
              key
          }
      }
  };

  Ok(password)
}

fn get_env_path() -> Result<PathBuf, Box<dyn Error>> {
    let home_dir = dirs::home_dir().ok_or("Failed to get home directory")?;
    let env_path = home_dir.join(ENV_FILE);
    Ok(env_path)
}

fn write_error_to_file(err: &dyn Error) -> Result<(), Box<dyn Error>> {
    let home_dir = dirs::home_dir().ok_or("Failed to get home directory")?;
    let dir_path = home_dir.join(".ask-cli");
    fs::create_dir_all(&dir_path)?;
    let file_path = dir_path.join(".error_log");

    let mut error_msg = format!("An error occurred: {}\n", err);
    if file_path.exists() {
        error_msg.push_str(&fs::read_to_string(&file_path)?);
    }
    fs::write(file_path, error_msg)?;
    
    Ok(())
}