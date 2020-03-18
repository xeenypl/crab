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
        let rep = attohttpc::get(name).send().expect("Request fail!\n");
        if rep.is_success() {
             return rep.text().expect("");
        }
        panic!("Request failed!");
    }
}
