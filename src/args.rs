use clap::{App, Arg, SubCommand};

pub fn get_args() -> clap::ArgMatches<'static> {
    return App::new("crab - cli web scraper")
        .version("0.2.1")
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
}
