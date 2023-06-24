To implement cursor movement with arrow keys, we need to use a library that allows us to read input from the terminal. We can use the `termion` library for this purpose.

First, we need to add `termion` to our `Cargo.toml` file:

```toml
[dependencies]
termion = "1.5.6"
```

Then, we can modify the `loop` block in the `main` function to read input from the terminal and move the cursor accordingly:

```rust
use termion::event::Key;
use termion::input::TermRead;

// ...

loop {
    print!("> ");
    let _ = std::io::stdout().flush();

    let mut question = String::new();
    let stdin = std::io::stdin();
    let mut stdin_locked = stdin.lock();
    let mut cursor_position = 0;

    for key in stdin_locked.keys() {
        match key? {
            Key::Char('\n') => break,
            Key::Char(c) => {
                question.insert(cursor_position, c);
                cursor_position += 1;
            }
            Key::Backspace => {
                if cursor_position > 0 {
                    question.remove(cursor_position - 1);
                    cursor_position -= 1;
                }
            }
            Key::Delete => {
                if cursor_position < question.len() {
                    question.remove(cursor_position);
                }
            }
            Key::Left => {
                if cursor_position > 0 {
                    cursor_position -= 1;
                }
            }
            Key::Right => {
                if cursor_position < question.len() {
                    cursor_position += 1;
                }
            }
            _ => {}
        }

        print!("\r> {}{}", question, termion::cursor::Goto((cursor_position + 2) as u16, 1));
        let _ = std::io::stdout().flush();
    }

    question = question.trim().to_string();

    // ...
}
```

Here, we use `stdin.lock()` to get a locked handle to the standard input stream, which allows us to read input from the terminal. We then use a `for` loop to iterate over the keys that are pressed, and update the `question` and `cursor_position` variables accordingly.

We also use the `termion::cursor::Goto` function to move the cursor to the correct position after each key press.

With these changes, the user should be able to move the cursor with the arrow keys while entering input in the REPL mode.