extern crate clap;
use clap::{App, Arg, SubCommand};

extern crate html5ever;
extern crate markup5ever_rcdom as rcdom;

use ansi_term;
use scraper;

use reqwest;
use reqwest::RequestBuilder;

use std::default::Default;
use std::fs;
use std::io;
use std::io::Read;
use std::iter::repeat;
use std::path::Path;
use std::string::String;

use html5ever::parse_document;
use html5ever::tendril::TendrilSink;
use rcdom::Handle;
use rcdom::NodeData;
use rcdom::RcDom;

fn get_content(name: &str, post: Option<&str>) -> String {
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

fn walk(indent: usize, handle: &Handle, no_colors: bool) {
    let node = handle;
    let tab = repeat(" ").take(indent).collect::<String>();
    match node.data {
        NodeData::Element {
            ref name,
            ref attrs,
            ..
        } => {
            if name.local.to_string() == "body"
                || name.local.to_string() == "head"
                || name.local.to_string() == "html"
            {
                for child in node.children.borrow().iter() {
                    walk(indent, child, no_colors);
                }
                return;
            }
            if no_colors {
                println!("{}{}: {}", tab, "tag", name.local.to_string());
                for attr in attrs.borrow().iter() {
                    println!(
                        "{} {}: {}",
                        tab,
                        attr.name.local.to_string(),
                        attr.value.to_string(),
                    );
                }
            } else {
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
            for child in node.children.borrow().iter() {
                walk(indent + 2, child, no_colors);
            }
        }

        _ => {
            for child in node.children.borrow().iter() {
                walk(indent, child, no_colors);
            }
        }
    }
}

fn print_dom(html: &str, no_colors: bool) {
    let dom = parse_document(RcDom::default(), Default::default())
        .from_utf8()
        .read_from(&mut html.as_bytes())
        .unwrap();
    walk(0, &dom.document, no_colors);
}

fn main() {
    let matches = App::new("crab - cli web scraper")
        .version("0.2.0")
        .author("xeeny <me@xeeny.pl>")
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
        .arg(
            Arg::with_name("no-colors")
                .short("-n")
                .long("no-colors")
                .help("show DOM without colors"),
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
                )
                .arg(
                    Arg::with_name("row")
                        .short("r")
                        .long("row")
                        .help("print row content of tag"),
                )
                .arg(
                    Arg::with_name("no-colors")
                        .short("-n")
                        .long("no-colors")
                        .help("show DOM without colors"),
                ),
        )
        .get_matches();

    let soures: String = matches.value_of("URL").unwrap().to_string();
    let comment: String = get_content(&soures, matches.value_of("post"));
    let no_colors: bool = matches.is_present("no-colors");

    if let Some(matches) = matches.subcommand_matches("get") {
        let error_prefix = &ansi_term::Color::Red.paint("[Selector Errer]: ");
        let no_colors: bool = matches.is_present("no-colors") || no_colors;
        let selector: String = matches.value_of("SELECTOR").unwrap().to_string();
        let document = scraper::Html::parse_document(&comment);
        let selector = scraper::Selector::parse(&selector).expect(&format!(
            "{}wrong selector:\n\t {}",
            error_prefix, &selector
        ));
        let mut limit: usize = matches.value_of("limit").unwrap_or("100").parse().unwrap();
        'print_loop: for elem in document.select(&selector) {
            if matches.is_present("attribute") {
                let attr = matches.value_of("attribute").unwrap();
                println!("{}", elem.value().attr(attr).unwrap_or(""));
            } else {
                if matches.is_present("row") {
                    println!("{}", elem.text().collect::<Vec<_>>().join(""));
                } else {
                    print_dom(&elem.inner_html(), no_colors);
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
            print_dom(&elem.inner_html(), no_colors);
        }
    }
}
