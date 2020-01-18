use reqwest::RequestBuilder;

use reqwest;
use std::fs;
use std::io;
use std::io::Read;
use std::path::Path;
use std::string::String;

pub fn get(name: &str, post: Option<&str>) -> String {
    let error_prefix = &ansi_term::Color::Red.paint("[Connection Errer]: ");
    if name == "stdin" {
        let mut buf = String::new();
        io::stdin()
            .lock()
            .read_to_string(&mut buf)
            .expect(&format!("{}problen with open stdin.", error_prefix));
        return buf;
    } else if Path::new(name).exists() {
        return fs::read_to_string(name)
            .expect(&format!("{}problen with open {}.", error_prefix, name));
    } else {
        let c = reqwest::Client::new();
        let res: RequestBuilder;
        match post {
            Some(s) => {
                res = c.post(name).form(s);
            }
            None => {
                res = c.get(name);
            }
        }
        return res
            .send()
            .expect(&format!("{}failed request to {}", error_prefix, name))
            .text()
            .expect(&format!("{}failed request to text", error_prefix));
    }
}
