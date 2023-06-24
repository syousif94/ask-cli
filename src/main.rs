pub mod exec_prompt;
pub mod secure_storage;
pub mod ai;


use clap::{Args, Parser, Subcommand};
use exec_prompt::execute_prompt_file;

use std::error::Error;
use std::io::{Write};
use std::fs::File;
use keyring::{Entry};

use async_openai::{
    types::{ChatCompletionRequestMessageArgs, ChatCompletionRequestMessage, Role},
};
use ai::{create_open_ai_client, stream_request};
use std::path::Path;


#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct App {
    question: Option<String>,
    #[clap(subcommand)]
    command: Option<Command>,
}

#[derive(Debug, Subcommand)]
enum Command {
    Key(KeyArgs),
    Clear {}
}

#[derive(Debug, Args)]
struct KeyArgs {
    key: String,
}

async fn answer(messages: &mut Vec<ChatCompletionRequestMessage>) -> Result<String, Box<dyn Error>> {

    let client = create_open_ai_client().unwrap();

    let answer = stream_request(&client, messages, |_s|{}).await?;

    Ok(answer)
}

fn save_to_log_file(messages: &Vec<ChatCompletionRequestMessage>) -> Result<(), Box<dyn Error>> {
    let mut log_text = String::new();

    for message in messages.clone() {
        match message.role {
            Role::User => {
                // append to output_text
                log_text += &format!("**user**: {}\n", message.content);
            }
            Role::Assistant => {
                log_text += &format!("**ai**: {}\n", message.content);
            }
            _ => {}
        }
    }

    let mut file = File::create("log.md").expect("Could not create log file");

    file.write_all(log_text.as_bytes()).expect("Could not write log file");

    Ok(())
}

async fn handle_question(question: &str, messages: &mut Vec<ChatCompletionRequestMessage>) -> Result<(), Box<dyn Error>> {
    messages.push(
        ChatCompletionRequestMessageArgs::default()
            .content(String::from(question))
            .role(Role::User)
            .build()?
    );

    let response = answer(&mut messages.clone()).await?;

    println!("");

    messages.push(
        ChatCompletionRequestMessageArgs::default()
            .content(response)
            .role(Role::Assistant)
            .build()?
    );

    Ok(())
}

async fn handle_prompt_arg(prompt_path: &str) -> Result<(), Box<dyn Error>> {
    execute_prompt_file(prompt_path).await?;

    Ok(())
}

fn handle_key_command(key_args: &KeyArgs) -> Result<(), Box<dyn Error>> {
    let entry = Entry::new("ask_cli", "open_ai")?;
    entry.set_password(&key_args.key.trim())?;
    println!("Key saved");

    Ok(())
}

fn handle_clear_command() -> Result<(), Box<dyn Error>> {
    let entry = Entry::new("ask_cli", "open_ai")?;
    entry.delete_password()?;
    println!("Key cleared");

    Ok(())
}

fn is_valid_file_path(path_str: &str) -> bool {
    let path = Path::new(path_str);
    path.exists() && path.is_file()
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let args = App::parse();

    let mut messages: Vec<ChatCompletionRequestMessage> = vec![];

    match args.command {
        Some(Command::Key(key_args)) => {
            handle_key_command(&key_args)?;
        }
        Some(Command::Clear {}) => {
            handle_clear_command()?;
        }
        None => {
            match args.question {
                Some(question) => {
                    if is_valid_file_path(&question) {
                        handle_prompt_arg(&question).await?;
                        return Ok(())
                    }
                    handle_question(&question, &mut messages).await?;
                }
                None => {
                    println!("What can I assist you with?");
                    loop {
                        print!("> ");
                        let _ = std::io::stdout().flush();
                        let mut question = String::new();
                        std::io::stdin().read_line(&mut question)?;

                        question = question.trim().to_string();

                        match question.as_str() {
                            "exit" => break,
                            "log" => {
                                save_to_log_file(&messages)?;
                                continue;
                            }
                            _ => {}
                        }

                        handle_question(&question, &mut messages).await?;
                    }
                }
            }
        }
    }

    Ok(())
}