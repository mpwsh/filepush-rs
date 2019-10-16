extern crate clap;

use clap::{App, Arg};
use transfer::upload;

fn main() {
    let matches = App::new("transfer")
        .version("0.1")
        .author("Harsh Shandilya <msfjarvis@gmail.com>")
        .about("Rust application for uploading files to transfer.sh")
        .arg(
            Arg::with_name("file")
                .short("f")
                .long("file")
                .value_name("FILE")
                .help("File to upload")
                .required(true)
                .takes_value(true),
        )
        .get_matches();

    let file = matches.value_of("file").unwrap();
    match upload(file) {
        Ok(url) => println!("{}", url),
        Err(err) => panic!("Error: {}", err),
    };
}
