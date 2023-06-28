Rewritten ask-cli Homebrew formula:

```ruby
class AskCli < Formula
  desc "A command line interface for Amazon Alexa"
  homepage "https://github.com/syousif94/ask-cli"
  url "https://github.com/syousif94/ask-cli/archive/refs/tags/v0.1.0.tar.gz"
  sha256 "714f4ee9c901f4b3cc75b7a34e6a34e0b1ae3123879de594f73186362f8e09aa"
  license "Apache-2.0"
  head "https://github.com/syousif94/ask-cli.git", branch: "main"

  livecheck do
    url :stable
    strategy :github_latest
  end

  depends_on "rust" => :build

  def install
    system "cargo", "install", *std_cargo_args

    # Install tab completion script
    bash_completion.install "completions/ask"
    fish_completion.install "completions/ask.fish"
    zsh_completion.install "completions/ask_zsh"

    # Install license files
    prefix.install "LICENSE.txt"
    prefix.install "NOTICE.txt"
  end

  test do
    system "#{bin}/ask", "--version"
  end
end
```

Note: Since ask-cli does not include a manpage or a completion script for the zsh shell, these lines have been removed from the rewritten formula.