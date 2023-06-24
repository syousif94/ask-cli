```diff
--- a/main.rs
+++ b/main.rs
@@ -1,5 +1,7 @@
 pub mod watch_prompt;
 pub mod secure_storage;
 pub mod ai;
+use termion::event::Key;
+use termion::input::TermRead;
 
 use clap::{Args, Parser, Subcommand};
 use watch_prompt::watch_prompt;
@@ -14,6 +16,7 @@ use keyring::{Entry};
 use async_openai::{
     types::{ChatCompletionRequestMessageArgs, ChatCompletionRequestMessage, Role},
 };
+use termion::cursor;
 use ai::{create_open_ai_client, stream_request};
 
 
@@ -28,6 +31,7 @@ struct App {
     #[arg(short, long)]
     prompt: Option<String>,
     #[clap(subcommand)]
+    command: Option<Command>,
     cursor_position: Option<usize>,
 }
 
@@ -36,6 +40,7 @@ enum Command {
     Key(KeyArgs),
     Clear {},
 }
+
 #[derive(Debug, Args)]
 struct KeyArgs {
     key: String,
@@ -44,6 +49,7 @@ async fn answer(messages: &mut Vec<ChatCompletionRequestMessage>) -> Result<St
     let client = create_open_ai_client().unwrap();
 
     let answer = stream_request(&client, messages).await?;
+
     Ok(answer)
 }
 
@@ -54,6 +60,7 @@ fn save_to_log_file(messages: &Vec<ChatCompletionRequestMessage>) -> Result<()>
     let mut log_text = String::new();
 
     for message in messages.clone() {
+
         match message.role {
             Role::User => {
                 // append to output_text
@@ -68,6 +75,7 @@ fn save_to_log_file(messages: &Vec<ChatCompletionRequestMessage>) -> Result<()>
     Ok(())
 }
 
+
 async fn handle_question(question: &str, messages: &mut Vec<ChatCompletionRequestMessage>, cursor_position: &mut usize) -> Result<(), Box<dyn Error>> {
     messages.push(
         ChatCompletionRequestMessageArgs::default()
@@ -77,6 +85,7 @@ async fn handle_question(question: &str, messages: &mut Vec<ChatCompletionReque
             .build()?
     );
 
+
     let response = answer(&mut messages.clone()).await?;
 
     println!("");
@@ -87,6 +96,7 @@ async fn handle_question(question: &str, messages: &mut Vec<ChatCompletionReque
             .build()?
     );
 
+
     Ok(())
 }
 
@@ -94,6 +104,7 @@ async fn handle_prompt_arg(prompt_path: &str) -> Result<(), Box<dyn Error>> {
     watch_prompt(prompt_path).await?;
 
     Ok(())
+}
 
 fn handle_key_command(key_args: &KeyArgs) -> Result<(), Box<dyn Error>> {
     let entry = Entry::new("ask_cli", "open_ai")?;
@@ -105,6 +116,7 @@ fn handle_clear_command() -> Result<(), Box<dyn Error>> {
     Ok(())
 }
 
+
 #[tokio::main]
 async fn main() -> Result<(), Box<dyn Error>> {
     let mut args = App::parse();
@@ -112,6 +124,7 @@ async fn main() -> Result<(), Box<dyn Error>> {
     let mut messages: Vec<ChatCompletionRequestMessage> = vec![];
     let mut cursor_position = 0;
 
+
     if let Some(prompt) = args.prompt {
         handle_prompt_arg(&prompt).await?;
         return Ok(());
@@ -119,6 +132,7 @@ async fn main() -> Result<(), Box<dyn Error>> {
         Some(Command::Key(key_args)) => {
             handle_key_command(&key_args)?;
         }
+
         Some(Command::Clear {}) => {
             handle_clear_command()?;
         }
@@ -126,6 +140,7 @@ async fn main() -> Result<(), Box<dyn Error>> {
                 handle_question(&question, &mut messages, &mut cursor_position).await?;
             }
             None => {
+
                 println!("What can I assist you with?");
                 loop {
                     print!("> ");
@@ -133,6 +148,7 @@ async fn main() -> Result<(), Box<dyn Error>> {
                     let mut question = String::new();
                     std::io::stdin().read_line(&mut question)?;
 
+
                     question = question.trim().to_string();
 
                     match question.as_str() {
@@ -142,6 +158,7 @@ async fn main() -> Result<(), Box<dyn Error>> {
                             break,
                             "log" => {
                                 save_to_log_file(&messages)?;
+
                                 continue;
                             }
                             _ => {}
@@ -149,6 +166,7 @@ async fn main() -> Result<(), Box<dyn Error>> {
 
                             handle_question(&question, &mut messages, &mut cursor_position).await?;
                         }
+
                     }
                 }
             }
@@ -156,3 +174,32 @@ async fn main() -> Result<(), Box<dyn Error>> {
     }
 
     Ok(())
+}
+
+loop {
+    print!("> ");
+    let _ = std::io::stdout().flush();
+
+    let mut question = String::new();
+    let stdin = std::io::stdin();
+    let mut stdin_locked = stdin.lock();
+    let mut cursor_position = 0;
+
+    for key in stdin_locked.keys() {
+        match key? {
+            Key::Char('\n') => break,
+            Key::Char(c) => {
+                question.insert(cursor_position, c);
+                cursor_position += 1;
+            }
+            Key::Backspace => {
+                if cursor_position > 0 {
+                    question.remove(cursor_position - 1);
+                    cursor_position -= 1;
+                }
+            }
+            Key::Delete => {
+                if cursor_position < question.len() {
+                    question.remove(cursor_position);
+                }
+            }
+            Key::Left => {
+                if cursor_position > 0 {
+                    cursor_position -= 1;
+                }
+            }
+            Key::Right => {
+                if cursor_position < question.len() {
+                    cursor_position += 1;
+                }
+            }
+            _ => {}
+        }
+
+        print!("\r> {}{}", question, cursor::Goto((cursor_position + 2) as u16, 1));
+        let _ = std::io::stdout().flush();
+    }
+
+    question = question.trim().to_string();
+}