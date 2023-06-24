## Tests for watch_prompt.rs

```rust
use super::*;

#[tokio::test]
async fn test_generate_response() {
    let prompt = "Hello, how are you doing today?";
    let result = generate_response(prompt).await;

    assert!(result.is_ok());
}

#[test]
fn test_load_prompt_from_file() {
    let file_path = "./tests/data/sample-prompt.txt";
    let result = load_prompt_from_file(file_path);

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

#[test]
fn test_ensure_path_exists() {
    let non_exist_file_path = "./tests/data/nonexistent-dir/output.txt";
    let result = ensure_path_exists(non_exist_file_path);

    assert!(result.is_ok());

    let exist_file_path = "./tests/data/exist-dir/output.txt";
    let result = ensure_path_exists(exist_file_path);
    assert!(result.is_ok());
}
```