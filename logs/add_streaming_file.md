The error is most likely due to the fact that `answer` is declared as `String::new()` and you are trying to use the `+=` operator to append the value of `content`. To fix this error, you should change `answer` to be a `StringBuffer` or a `String` with the `+=` operator replaced with `.push_str(content)`. Here's an example:

```rust
use std::io::Write;

pub async fn stream_request<F>(
  client: &Client<OpenAIConfig>,
  messages: &mut Vec<ChatCompletionRequestMessage>,
  mut on_content: Option<F>,
) -> Result<String, Box<dyn Error>>
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
            answer.push_str(content);
            write!(lock, "{}", content).unwrap();
            on_content.as_mut().map(|c| c(content));
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

```