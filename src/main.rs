extern crate clap;
use clap::{App, Arg, SubCommand};

use ansi_term;
use reqwest;
use scraper;

use std::default::Default;
use std::fs;
use std::io;
use std::io::Read;
use std::iter::repeat;
use std::path::Path;
use std::string::String;

use html5ever::rcdom::{Handle, NodeData, RcDom};
use html5ever::tendril::TendrilSink;

fn get_content(name: &str, post: Option<&str>) -> String {
    if name == "stdin" {
        let mut buf = String::new();
        io::stdin()
            .lock()
            .read_to_string(&mut buf)
            .expect(&ansi_term::Color::Red.paint(format!("problen with open {}.", name)));
        return buf;
    } else if Path::new(name).exists() {
        return fs::read_to_string(name).expect(&format!("problem with {}", name));
    } else {
        match post {
            Some(s) => {
                let c = reqwest::Client::new();
                let mut res = c
                    .post(name)
                    .form(s)
                    .send()
                    .expect(&format!("failed request to {}", name));
                return res.text().unwrap();
            }
            None => {
                return reqwest::get(name)
                    .expect(&format!("failed request to {}", name))
                    .text()
                    .unwrap()
            }
        }
    }
}

fn walk(indent: usize, handle: &Handle) {
    let node = handle;
    let tab = repeat(" ").take(indent).collect::<String>();
    match node.data {
        NodeData::Element {
            ref name,
            ref attrs,
            ..
        } => {
            println!(
                "{}{}: {}",
                tab,
                &ansi_term::Color::Cyan.paint("tag"),
                &ansi_term::Color::Red.paint(name.local.to_string())
            );
            for attr in attrs.borrow().iter() {
                println!(
                    "{} {}: {}",
                    tab,
                    &ansi_term::Color::Blue.paint(attr.name.local.to_string()),
                    &ansi_term::Color::Green.paint(attr.value.to_string()),
                );
            }
        }

        _ => {}
    }

    for child in node.children.borrow().iter() {
        walk(indent + 2, child);
    }
}

fn print_dom(html: &str) {
    let dom = html5ever::parse_document(RcDom::default(), Default::default())
        .from_utf8()
        .read_from(&mut html.as_bytes())
        .unwrap();
    walk(0, &dom.document);
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
    let comment: String = get_content(&soures, matches.value_of("post"));

    if let Some(matches) = matches.subcommand_matches("get") {
        let selector: String = matches.value_of("SELECTOR").unwrap().to_string();
        let document = scraper::Html::parse_document(&comment);
        let selector = scraper::Selector::parse(&selector)
            .expect(&ansi_term::Color::Red.paint(&format!("wrong selector:\n\t {}", &selector)));
        let mut limit: usize = matches.value_of("limit").unwrap_or("100").parse().unwrap();
        'print_loop: for elem in document.select(&selector) {
            if matches.is_present("attribute") {
                if !matches.is_present("show-dom") {
                    let attr = matches.value_of("attribute").unwrap();
                    println!("{}", elem.value().attr(attr).unwrap_or(""));
                } else {
                    print_dom(&elem.html());
                }
            } else {
                if !matches.is_present("show-dom") {
                    println!("{}", elem.text().collect::<Vec<_>>().join(""));
                } else {
                    print_dom(&elem.html());
                }
            }
            limit -= 1;
            if limit == 0 {
                break 'print_loop;
            }
        }
    } else {
        let document = scraper::Html::parse_document(&comment);
        let selector = scraper::Selector::parse("body").unwrap();
        for elem in document.select(&selector) {
            print_dom(&elem.html());
        }
    }
}
