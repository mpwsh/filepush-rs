# filepush-rs


Small Rust library to upload files to [filepush.co](https://filepush.co).
I just edited the [transfer-rs](https://crates.io/crates/transfer-rs) library to point to filepush, as both use Curl to upload files, so all the credit goes to his creator [Harsh Shandilya](https://github.com/msfjarvis)

## Usage

```rust
extern crate transfer;
use transfer::upload;

fn main() {
    match upload("Cargo.toml") {
        Ok(url) => println!("{}", url),
        Err(err) => panic!("Error: {}", err),
    };
}
```

A simple CLI application is provided in the [examples](examples/) directory.

## Building

- Install rustc using [RustUp](https://rustup.rs)
- Run `cargo build --release`
