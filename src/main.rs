extern crate clap;
use clap::{App, Arg, SubCommand};

use ansi_term;

use html2text;
use reqwest;
use scraper;

use std::io;
use std::io::Read;

fn get_content(name: &str) -> String {
    if name == "stdin" {
        let mut buf = String::new();
        io::stdin()
            .lock()
            .read_to_string(&mut buf)
            .expect(&ansi_term::Color::Red.paint(format!("problen with open {}.", name)));
        return buf;
    } else {
        return reqwest::get(name)
            .expect(&format!("felie reqwest to {}", name))
            .text()
            .unwrap();
    }
}

fn main() {
    let matches = App::new("crab - cli web scraper")
        .version("0.1.0")
        .author("xeeny <me@xeeny.pl>")
        .arg(
            Arg::with_name("cookies")
                .short("c")
                .long("cookies")
                .value_name("COOKIES_FILE")
                .help("Get localization of cookies file")
                .takes_value(true),
        )
        .arg(
            Arg::with_name("width")
                .short("w")
                .long("width")
                .value_name("WIDTH")
                .help("how wide display is")
                .takes_value(true),
        )
        .arg(
            Arg::with_name("show-dom")
                .short("s")
                .long("show-dom")
                .help("show document object model structure of the page"),
        )
        .arg(
            Arg::with_name("post")
                .short("p")
                .long("post")
                .value_name("ARGS")
                .help("Gets argument for the post method")
                .takes_value(true),
        )
        .arg(
            Arg::with_name("URL")
                .help("URL to site")
                .required(true)
                .index(1),
        )
        .subcommand(
            SubCommand::with_name("get")
                .about("prints COUNT found results")
                .arg(
                    Arg::with_name("show-dom")
                        .short("s")
                        .long("show-dom")
                        .help("show document object model structure of the page"),
                )
                .arg(
                    Arg::with_name("SELECTOR")
                        .help("which css selector")
                        .required(true)
                        .index(1),
                )
                .arg(
                    Arg::with_name("limit")
                        .short("l")
                        .long("limit")
                        .value_name("LIMIT")
                        .help("how meny result.")
                        .takes_value(true),
                )
                .arg(
                    Arg::with_name("attribute")
                        .short("a")
                        .long("attribute")
                        .value_name("ATTRIBUTE")
                        .help("which html attribute.")
                        .takes_value(true),
                ),
        )
        .get_matches();

    let soures: String = matches.value_of("URL").unwrap().to_string();
    let width: String = matches.value_of("width").unwrap_or("80").to_owned();
    let comment: String = get_content(&soures);

    if let Some(matches) = matches.subcommand_matches("get") {
        let selector: String = matches.value_of("SELECTOR").unwrap().to_string();
        let document = scraper::Html::parse_document(&comment);
        let selector = scraper::Selector::parse(&selector)
            .expect(&ansi_term::Color::Red.paint(&format!("wrong selector:\n\t {}", &selector)));
        for elem in document.select(&selector) {
            if matches.is_present("attribute") {
                if !matches.is_present("show-dom") {
                    let attr = matches.value_of("attribute").unwrap();
                    println!("{}", elem.value().attr(attr).unwrap_or(""));
                } else {
                    unimplemented!();
                }
            } else {
                if !matches.is_present("show-dom") {
                    println!("{}", elem.text().collect::<Vec<_>>().join(""));
                } else {
                    unimplemented!();
                }
            }
        }
    } else {
        if !matches.is_present("show-dom") {
            println!(
                "{}",
                html2text::from_read(&mut comment.as_bytes(), width.parse().unwrap())
            );
        } else {
            unimplemented!();
        }
    }
}
