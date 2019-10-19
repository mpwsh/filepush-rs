# filepush-rs


Small Rust library to upload and download files from/to [filepush.co](https://filepush.co).
I forked the [transfer-rs](https://crates.io/crates/transfer-rs) library and added download capabilities as well.
As both use Curl to upload files, so 
Transfer-rs lib author is [Harsh Shandilya](https://github.com/msfjarvis)

## Usage

```rust
extern crate filepush;
use filepush::{upload, download};

fn main() {
    match upload("Cargo.toml") {
        Ok(url) => println!("{}", url),
        Err(err) => panic!("Error: {}", err),
    };
    match download("https://filepush.co/9LY9/test1") {
        Ok(url) => println!("{}", url),
        Err(err) => panic!("Error: {}", err),
    };
}
```

A simple CLI application is provided in the [examples](examples/) directory.

## Building

- Install rustc using [RustUp](https://rustup.rs)
- Run `cargo build --release`

## Example usage
- cargo run --example cli_client -- -u yourfile
- cargo run --example cli_client -- -d yourlink
