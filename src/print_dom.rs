extern crate html5ever;
extern crate markup5ever_rcdom as rcdom;

use html5ever::parse_document;
use html5ever::tendril::TendrilSink;
use rcdom::Handle;
use rcdom::NodeData;
use rcdom::RcDom;
use std::default::Default;
use std::iter::repeat;

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

pub fn print(html: &str, no_colors: bool) {
    let dom = parse_document(RcDom::default(), Default::default())
        .from_utf8()
        .read_from(&mut html.as_bytes())
        .unwrap();
    walk(0, &dom.document, no_colors);
}
