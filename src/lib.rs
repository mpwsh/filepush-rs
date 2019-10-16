extern crate reqwest;

use reqwest::header::USER_AGENT;
use reqwest::Client;
use reqwest::Error;
use reqwest::Response;
use std::fs::File;
use std::io::Read;
use std::path::Path;

pub fn upload(file_path: &str) -> Result<String, Box<dyn std::error::Error>> {
    let client = Client::new();
    let path = Path::new(file_path);
    let basename = match path.file_name().unwrap().to_str() {
        Some(s) => s,
        None => {
            panic!("Failed to determine path");
        }
    };
    let url = &format!("https://filepush.co/upload/{}", basename);
    let file = File::open(file_path)?;
    let res = make_request(client, url, file);
    return match res {
        Ok(mut r) => {
            let mut body = String::new();
            r.read_to_string(&mut body)?;
            Ok(body)
        },
        Err(e) => Err(e.into()),
    }
}

fn make_request(client: Client, url: &str, file: File) -> Result<Response, Error> {
    client
        .put(url)
        .body(file)
        .header(USER_AGENT, "curl/7.58.0") // Without faking the user agent, we'll get a webpage.
        .send()
}
