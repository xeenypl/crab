extern crate clap;
use clap::{App, Arg, SubCommand};

use ansi_term;

use html2text;
use reqwest;
use scraper;

use std::io;
use std::io::Read;

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
}
