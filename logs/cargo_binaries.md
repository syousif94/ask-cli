To build binaries for a CLI app for Apple Silicon, x86 OSX, and Linux using Cargo, and package them for distribution on Homebrew, you can follow the steps below:

1. Clone the github repo for the cli app you want to build and package:
```
git clone https://github.com/syousif94/ask-cli.git
```

2. Navigate to the root directory of the project:
```
cd ask-cli
```

3. Create a `release` build of the app for each platform using Cargo:
```
cargo build --manifest-path=cli/Cargo.toml --release --target=x86_64-apple-darwin
cargo build --manifest-path=cli/Cargo.toml --release --target=aarch64-apple-darwin
cargo build --manifest-path=cli/Cargo.toml --release --target=x86_64-unknown-linux-gnu
```
Note: You can also build for other platforms if desired.

4. Create a directory for the Homebrew Formula, and copy the built binaries into it:
```
mkdir ask-cli-formula
cp target/x86_64-apple-darwin/release/ask ask-cli-formula/
cp target/aarch64-apple-darwin/release/ask ask-cli-formula/
cp target/x86_64-unknown-linux-gnu/release/ask ask-cli-formula/
```
Note: The name of the binary file `ask` may differ for the app you are building.

5. Create a file called `ask-cli.rb` in the Homebrew Formula directory, and paste the following code into it:
```
class AskCli < Formula
  desc "Command line interface for interacting with AWS services and resources"
  homepage "https://github.com/syousif94/ask-cli"
  url "https://github.com/syousif94/ask-cli/releases/latest/download/ask-cli-VERSION.tar.gz"
  sha256 "PACKAGE_SHA256"

  depends_on "python"

  def install
    bin.install "ask"
  end

  test do
    system "#{bin}/ask", "--version"
  end
end
```
Note: Update the values of `desc`, `homepage`, `url`, and `sha256` accordingly for the app you are building.

6. Calculate the SHA256 hash of each built binary, and update the value of `PACKAGE_SHA256` in the `ask-cli.rb` file accordingly:
```
shasum -a 256 target/x86_64-apple-darwin/release/ask
shasum -a 256 target/aarch64-apple-darwin/release/ask
shasum -a 256 target/x86_64-unknown-linux-gnu/release/ask
```

7. Zip the Homebrew Formula directory:
```
cd ..
zip -r ask-cli-formula.zip ask-cli-formula/
```

8. Upload the zipped Homebrew Formula directory to a public location, such as an S3 bucket.

9. Create or update the Homebrew Tap for the app you are building, and reference the zipped Homebrew Formula directory as the source:
```
tap "my-github-username/homebrew-my-tap" do
  ...
  url "https://my-s3-bucket-url/ask-cli-formula.zip"
  ...
end
```
Note: Replace `my-github-username/homebrew-my-tap` with the appropriate values for your Tap, and update the URL accordingly.