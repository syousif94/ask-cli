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

`brew install ask-cli`

### TODO / Nice to have

[] timestamps for exported chat log
[] enter chat after passing prompt template
[] watch_dir in prompt template settings
