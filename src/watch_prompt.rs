use std::collections::HashMap;
use std::fs::{self, DirBuilder, read_to_string};
use regex::Regex;
use std::error::Error;
use async_openai::{
  types::{ChatCompletionRequestMessageArgs, ChatCompletionRequestMessage, Role}
};
use std::path::Path;

use crate::ai::{stream_request, create_open_ai_client};

pub async fn watch_prompt(path: &str) -> Result<String, Box<dyn Error>> {
    let prompt = load_prompt_from_file(path).expect("Failed to load prompt from file");

    let settings = parse_settings(&prompt);
    let replaced_prompt = replace_variables(&prompt, &settings);

    let answer = generate_response(&replaced_prompt).await?;

    if let Some(log_file_value) = settings.get("log_file") {
      ensure_path_exists(log_file_value).expect("Unable to create path");
      fs::write(log_file_value, &answer).expect("Unable to write file");
    }

    Ok(answer)
}

fn ensure_path_exists(file_path: &str) -> Result<(), String> {
  let dir_path = Path::new(file_path).parent().unwrap();
  let mut dir_builder = DirBuilder::new();
  dir_builder.recursive(true);
  match dir_builder.create(dir_path) {
      Ok(_) => Ok(()),
      Err(e) => Err(format!("{}", e)),
  }
}

async fn generate_response(prompt: &str) -> Result<String, Box<dyn Error>> {

    let mut messages: Vec<ChatCompletionRequestMessage> = vec![];

    messages.push(
      ChatCompletionRequestMessageArgs::default()
            .content(prompt)
            .role(Role::System)
            .build()?
    );

    let client = create_open_ai_client().unwrap();

    let answer = stream_request(&client, &mut messages).await?;

    Ok(answer)
}

fn load_prompt_from_file(file_path: &str) -> Result<String, std::io::Error> {
  read_to_string(file_path)
}

fn parse_settings(prompt: &str) -> HashMap<String, String> {
  let mut settings = HashMap::new();
  let lines = prompt.lines().map(str::trim);

  for line in lines {
      if line.is_empty() || line == "=====" {
          break;
      }
      println!("line: {}\n", line);
      if let Some((key, value)) = line.split_once('=') {
          let key = key.trim();
          let value = value.trim();
          settings.insert(key.to_string(), value.to_string());
      }
  }

  settings
}

fn replace_variables(prompt: &str, variables: &HashMap<String, String>) -> String {
  let mut replaced_prompt = prompt.to_string();
  let re = Regex::new(r"\{\{(.*?)\}\}").unwrap();

  for capture in re.captures_iter(prompt) {
      
      if let Some(var_name) = capture.get(1) {
          let var_name = var_name.as_str();
          println!("var_name: {:?}\n", var_name);

          // if var_name starts with file:, load the file and replace the variable with the file contents

          if var_name.starts_with("file: ") {
            let file_path = &var_name[6..]; // Remove the "file: " prefix
            let contents = match read_to_string(file_path) {
                Ok(file_contents) => file_contents,
                Err(error) => {
                    eprintln!("Failed to read file {}: {}", file_path, error);
                    continue;
                }
            };

            replaced_prompt = replaced_prompt.replace(&capture[0], &contents);

          }else if let Some(var_value) = variables.get(var_name) {
            replaced_prompt = replaced_prompt.replace(&capture[0], var_value);
          }
      }
  }

  replaced_prompt
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_generate_response() {
        let prompt = "Hello, how are you doing today?";
        let result = tokio_test::block_on(generate_response(prompt));

        assert!(result.is_ok());
    }

    #[test]
    fn test_parse_settings() {
        let prompt = "log_file=output.txt\nname=John\nemail=john@example.com\n=====\nHello, {{name}}. How can I help you today?";
        let result = parse_settings(prompt);

        assert_eq!(result.get("log_file"), Some(&"output.txt".to_string()));
        assert_eq!(result.get("name"), Some(&"John".to_string()));
        assert_eq!(result.get("email"), Some(&"john@example.com".to_string()));
    }

    #[test]
    fn test_replace_variables() {
        let prompt = "Hello, {{name}}. {{question1}} {{name}}? {{question2}}";
        let mut variables = HashMap::new();
        variables.insert("name".to_string(), "John".to_string());
        variables.insert("question1".to_string(), "How are you".to_string());
        variables.insert("question2".to_string(), "Can I help you with anything?".to_string());

        let result = replace_variables(prompt, &variables);

        let expected_output = "Hello, John. How are you John? Can I help you with anything?".to_string();
        assert_eq!(result, expected_output);
    }
}