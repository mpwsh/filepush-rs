extern crate clap;

use clap::{App, Arg};
use filepush::{upload, download};

fn main() {
    let matches = App::new("transfer")
        .version("0.1")
        .author("Mariano Arellano <im@mariano.pw>")
        .about("Rust application for uploading and downloading files to/from filepush.co")
        .arg(
            Arg::with_name("upload")
                .short("u")
                .long("upload")
                .value_name("FILE")
                .help("File to upload")
                .required(false)
                .takes_value(true),
        )
        .arg(
            Arg::with_name("download")
                .short("d")
                .long("download")
                .value_name("URL")
                .help("Url to download")
                .required(false)
                .takes_value(true),
        )
        .get_matches();

    if let Some(file) = matches.value_of("upload") {
    match upload(file) {
        Ok(url) => println!("{}", url),
        Err(err) => panic!("Error: {}", err),
    };
    }
    if let Some(url) = matches.value_of("download") {
    match download(url) {
        Ok(fname) => println!("Downloaded file: {}", fname),
        Err(err) => panic!("Error: {}", err),
    };
    }
}
