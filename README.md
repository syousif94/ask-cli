## ASK CLI

### Usage

Chat Mode

`ask`

(type log to export the log)

Pipe

`ask "how do I create a new git branch?" | tee git_new_branch.md`

Prompt Template

`ask prompts/cargo_to_homebrew.txt`

#### Example Prompt Template

```
ext = .md
=====

Write tests for the following main.rs file

{{file: src/main.rs}}
```

### Installation

```bash
brew tap syousif94/ask
brew install ask
```

### TODO / Nice to have

- [ ] timestamps for exported chat log
- [ ] use AI generated summary as file names for logs
- [ ] enter chat after passing prompt template
- [ ] watch_dir in prompt template settings
- [ ] implement something like log 1 in the chat to only get the recent X logs
- [ ] add arrow key and multi line editing to the chat interface with termion or tui
- [ ] stream the chat to a file
- [ ] add tools to the chat system message
