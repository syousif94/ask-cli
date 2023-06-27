class AskCli < Formula
  desc "Command-line interface to interact with Alexa Skills Kit"
  homepage "https://github.com/syousif94/ask-cli"
  url "https://github.com/syousif94/ask-cli/archive/refs/tags/v0.1.0.tar.gz"
  sha256 "ec686bfae5484c96d7c1c9fb53a4b7b51fbee6c6c926a449f86f2f23831089d0"
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