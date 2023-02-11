use std::io::Write;

use pulldown_cmark::{CodeBlockKind, Event, Parser, Tag};

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

pub fn print_readme(parser: Parser, out: &mut dyn Write) {
    // TODO: List Types can not be recognized => Always prints unordered List
    // TODO: Indented Code Block is converted to fenced Code Block
    // TODO: Link Type is ignored
    for p in parser {
        match p {
            Event::Text(text) => write!(out, "{}", text),
            Event::Html(node) => write!(out, "{}", node),
            // NOTE: Event::Code is only used for Inline Code
            Event::Code(text) => writeln!(out, "`{}`", text),
            Event::Rule => write!(out, "\n---\n"),
            Event::TaskListMarker(marked) => write!(
                out,
                "{}",
                match marked {
                    true => "[X] ",
                    false => "[ ] ",
                }
            ),
            Event::SoftBreak | Event::HardBreak => writeln!(out),
            Event::FootnoteReference(label) => write!(out, "{}", label),
            // Event::Start(Tag::Paragraph) | Event::End(Tag::Paragraph) => write!(out, "\n\n"),
            Event::Start(Tag::Heading(level, _, _)) => {
                write!(out, "\n{}", "#".repeat(level as usize))
            }
            Event::Start(Tag::BlockQuote) => write!(out, "> "),
            Event::Start(Tag::CodeBlock(CodeBlockKind::Indented))
            | Event::End(Tag::CodeBlock(CodeBlockKind::Indented))
            | Event::End(Tag::CodeBlock(CodeBlockKind::Fenced(_))) => write!(out, "```"),
            Event::Start(Tag::CodeBlock(CodeBlockKind::Fenced(info))) => {
                writeln!(out, "```{}", info)
            }
            Event::Start(Tag::Link(_, url, title)) => writeln!(out, "[{}]({})", title, url),
            Event::Start(Tag::Image(_, url, title)) => writeln!(out, "![{}]({})", title, url),
            Event::Start(Tag::Emphasis) | Event::End(Tag::Emphasis) => write!(out, "*"),
            Event::Start(Tag::Strong) | Event::End(Tag::Strong) => write!(out, "**"),
            Event::Start(Tag::Item) => write!(out, "- "),
            Event::End(Tag::Item) => writeln!(out),
            Event::End(Tag::List(_)) | Event::End(Tag::Heading(_, _, _)) => writeln!(out),
            Event::Start(_) | Event::End(_) => Ok(()),
            _ => unimplemented!(),
        };
    }
}
