use ansi_term;
use scraper;

mod args;
mod get_content;
mod print_dom;

fn main() {
    let matches = args::get_args();
    let soures: String = matches.value_of("URL").unwrap().to_string();
    let comment: String = get_content::get(&soures, matches.value_of("post"));
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
                    print_dom::print(&elem.html(), no_colors);
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
            print_dom::print(&elem.inner_html(), no_colors);
        }
    }
}
