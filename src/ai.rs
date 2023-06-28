use async_openai::{
  types::{CreateChatCompletionRequestArgs,ChatCompletionRequestMessage},
  Client,
};
use std::error::Error;
use std::io::{stdout, Write};

use async_openai::config::OpenAIConfig;

use futures::StreamExt;

use crate::secure_storage::get_api_key;

pub fn create_open_ai_client() -> Result<Client<OpenAIConfig>, Box<dyn Error>> {
  let api_key = get_api_key().unwrap();

  let config = OpenAIConfig::new().with_api_key(api_key);

  let client = Client::with_config(config);

  Ok(client)
}

pub async fn stream_request<F>(client: &Client<OpenAIConfig>, messages: &mut Vec<ChatCompletionRequestMessage>, mut on_content: F) -> Result<String, Box<dyn Error>>
where
    F: FnMut(&str),
{
  let request = CreateChatCompletionRequestArgs::default()
        .model("gpt-3.5-turbo")
        .messages(messages.clone())
        .build()?;

    let mut stream = client.chat().create_stream(request).await?;

    let mut lock = stdout().lock();
    let mut answer = String::new();
    while let Some(result) = stream.next().await {
        match result {
            Ok(response) => {
                response.choices.iter().for_each(|chat_choice| {
                    if let Some(ref content) = chat_choice.delta.content {
                        answer += content.as_str();
                        write!(lock, "{}", content).unwrap();
                        on_content(content);
                    }
                });
            }
            Err(err) => {
                writeln!(lock, "error: {err}").unwrap();
            }
        }
        stdout().flush()?;
    }

    Ok(answer)
}