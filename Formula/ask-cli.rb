class AskCli < Formula
  desc "Command-line interface to interact with Alexa Skills Kit"
  homepage "https://github.com/syousif94/ask-cli"
  url "https://github.com/syousif94/ask-cli/archive/refs/tags/v0.1.0.tar.gz"
  sha256 "53fbee4f906d7b4ce2fa59132a9fab6b3873c90f4f79d3c9a377515271f166fe"
  license "MIT"
  head "https://github.com/syousif94/ask-cli.git", branch: "main"

  depends_on "rust" => :build

  def install
    system "cargo", "install", *std_cargo_args
  end

  test do
    system "#{bin}/ask", "--version"
  end
end