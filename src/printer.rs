use std::io::Write;
use std::{fmt::Display, io::Result};

use pulldown_cmark::{CodeBlockKind, Event, LinkType, Parser, Tag};

pub fn print_tags_raw(parser: Parser) {
    for c in parser {
        println!("{:?}", c);
    }
}

pub fn print_tags_indented(parser: Parser, show_start_end_tags: bool) {
    let mut indent_str = String::new();

    for c in parser {
        match c {
            Event::Start(Tag::List(..) | Tag::Table(..) | Tag::BlockQuote) => {
                if (show_start_end_tags) {
                    println!("{}{:?}", indent_str, c);
                }
                indent_str.push('\t');
            }
            Event::End(Tag::List(..) | Tag::Table(..) | Tag::BlockQuote) => {
                if (show_start_end_tags) {
                    println!("{}{:?}", indent_str, c);
                }
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
    let mut lists: Vec<Option<u64>> = Vec::new();
    let mut list_indent_str = String::new();
    for p in parser {
        match p {
            // NOTE: Footnotes and Table are not supported
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
            Event::Start(Tag::Heading(level, _, _)) => {
                write!(out, "\n{} ", "#".repeat(level as usize))
            }
            // Event::Start(Tag::Paragraph) | Event::End(Tag::Paragraph) => writeln!(out),
            Event::Start(Tag::BlockQuote) => write!(out, "> "),
            Event::Start(Tag::CodeBlock(CodeBlockKind::Indented))
            | Event::End(Tag::CodeBlock(CodeBlockKind::Indented))
            | Event::End(Tag::CodeBlock(CodeBlockKind::Fenced(_))) => write!(out, "```\n"),
            Event::Start(Tag::CodeBlock(CodeBlockKind::Fenced(info))) => {
                writeln!(out, "```{}", info)
            }
            Event::Start(Tag::List(lt)) => {
                if (!lists.is_empty()) {
                    list_indent_str.push('\t');
                }
                lists.push(lt);
                Ok(())
            }
            Event::End(Tag::List(_)) => {
                let _ = lists.pop();
                if (!lists.is_empty()) {
                    list_indent_str.pop();
                }
                Ok(())
            }
            Event::Start(Tag::Link(link_type, url, title)) => {
                print_link(out, link_type, &url, &title, true)
            }
            Event::Start(Tag::Image(link_type, url, title)) => {
                print_link(out, link_type, &url, &title, false)
            }
            Event::Start(Tag::Emphasis) | Event::End(Tag::Emphasis) => write!(out, "*"),
            Event::Start(Tag::Strong) | Event::End(Tag::Strong) => write!(out, "**"),
            Event::Start(Tag::Item) => {
                if let Some(list_type) = lists.pop() {
                    match list_type {
                        Some(i) => {
                            lists.push(list_type.map(|x| x + 1));
                            write!(out, "\n{}{}. ", list_indent_str, i)
                        }
                        None => {
                            lists.push(None);
                            write!(out, "\n{}- ", list_indent_str)
                        }
                    }
                } else {
                    panic!()
                }
            }
            Event::End(Tag::Item) => Ok(()),
            Event::End(Tag::Heading(_, _, _)) => writeln!(out),
            Event::Start(_) | Event::End(_) => Ok(()),
            _ => unimplemented!(),
        };
    }

    fn print_link(
        out: &mut dyn Write,
        link_type: LinkType,
        url: &dyn Display,
        title: &dyn Display,
        is_link: bool,
    ) -> Result<()> {
        if (!is_link) {
            write!(out, "!");
        }
        match link_type {
            LinkType::Inline => write!(out, "[{}]({})", title, url),
            LinkType::Reference | LinkType::ReferenceUnknown => write!(out, "[{}][{}]", title, url),
            LinkType::Collapsed | LinkType::CollapsedUnknown => write!(out, "[{}][]", url),
            LinkType::Shortcut | LinkType::ShortcutUnknown => write!(out, "[{}]", url),
            LinkType::Autolink | LinkType::Email => write!(out, "<{}>", url),
        }
    }
}
