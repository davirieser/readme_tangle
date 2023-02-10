
use pulldown_cmark::{Parser, Tag, Event};


pub fn print_parser_raw(parser: Parser) {
    for c in parser {
		println!("{:?}", c);
	}
}

pub fn print_parser_indented(parser: Parser, show_start_end_tags: bool) {
	let mut indent: usize = 0;
	let mut indent_str = String::new();

    for c in parser {
		match c {
			Event::Start(Tag::List(..) | Tag::Table(..) | Tag::BlockQuote) => {
				if (show_start_end_tags) {
					println!("{}{:?}", indent_str, c);
				}
				indent += 1;
				indent_str.push_str("\t");
			}
			Event::End(Tag::List(..) | Tag::Table(..) | Tag::BlockQuote) => {
				if (show_start_end_tags) {
					println!("{}{:?}", indent_str, c);
				}
				indent -= 1;
				indent_str.pop();
			}
			Event::Start(..) | Event::End(..) => {
				if (show_start_end_tags) {
					println!("{}{:?}", indent_str, c);
				}
			}
			_ => {
				println!("{}{:?}", indent_str, c);
			}
		}
    }
}