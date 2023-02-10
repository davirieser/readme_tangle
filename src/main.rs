#![allow(unused)]

use std::collections::HashMap;
use std::fmt::{Debug, Display, Formatter, Result as FmtResult};
use std::fs::File;
use std::io::{Read, Write};
use std::path::PathBuf;

use clap::Parser as ClapParser;
use pulldown_cmark::{html, CodeBlockKind, Event, Options, Parser, Tag, CowStr};

#[derive(ClapParser, Debug)]
#[command(
    about,
    author,
    version,
    long_about = None,
)]
/// Extract Source Code from README's similar to Emacs Org Mode.
///
/// See (https://orgmode.org/manual/Working-with-Source-Code.html)
struct Args {
    #[arg(short, long)]
    path: Vec<PathBuf>,
    #[arg(short, long, default_value_t = false)]
    auto_name: bool,
}

impl Args {
    fn new(path: Vec<PathBuf>, auto_name: bool) -> Self {
        Self { path, auto_name }
    }
}

#[derive(Debug)]
struct CodeBlockInfo {
    language: Option<String>,
    values: HashMap<String, String>,
	content:  String
}

enum CodeBlockInfoErrorType {
    IndentedCodeBlock,
}

impl TryFrom<CodeBlockKind<'_>> for CodeBlockInfo {
    type Error = CodeBlockInfoErrorType;

    fn try_from(cbk: CodeBlockKind) -> Result<Self, Self::Error> {
        match cbk {
            CodeBlockKind::Fenced(info) => {
                let mut infos = info.split(' ');
                let language = infos.next().map(str::to_owned);
                let values = HashMap::from_iter(
                    infos
                        .map(|s| s.split_once('='))
                        .filter(Option::is_some)
                        .map(Option::unwrap)
                        .map(|(s1, s2)| (s1.to_owned(), s2.to_owned())),
                );
                Ok(Self { language, values, content: String::new() })
            }
            _ => Err(CodeBlockInfoErrorType::IndentedCodeBlock),
        }
    }
}

fn get_code_blocks<'a>(input: &'a str) -> Box<dyn Iterator<Item = CodeBlockInfo> + 'a> {
    let mut acc = vec![];
    let mut arr = vec![];
	let mut current_info : Option<CodeBlockInfo> = None;
	let mut iter = Parser::new(input);

	let mut in_code_block = false;

	loop {
		match iter.next() {
			Some(tag) => {
				match tag {
					Event::Start(Tag::CodeBlock(cbk)) => {
						in_code_block = true;
						match CodeBlockInfo::try_from(cbk) {
							Ok(info) => current_info = Some(info),
							Err(e) => {}
						}
						continue;
					}
					Event::End(Tag::CodeBlock(cbk)) => {
						in_code_block = false;
						match current_info {
							Some(mut info) => {
								info.content = acc.iter().map(|x| match x {
									Event::Text(text) => text,
									_ => ""
								}).collect();
								acc = vec![];
								arr.push(info);
								current_info = None;
							}
							None => {}
						}
						continue;
					}
					_ => {}
				}
				if in_code_block {
					acc.push(tag);
				}
			}
			None => break
		}
	}

	if in_code_block { panic!("Unclosed Code Block"); }

	Box::new(arr.into_iter())
}

fn main() {
    let args = Args::parse();

	for p in args.path {
        if (p.exists()) {
            match File::open(p) {
                Ok(mut f) => {
                    let mut buf = String::new();
                    match f.read_to_string(&mut buf) {
                        Ok(_) => {
                            for c in get_code_blocks(&buf) {
								// TODO
								println!("{:?}", c);
                            }
                        }
                        Err(_) => {
                            println!("Error reading Contents");
                        }
                    }
                }
                Err(e) => {
                    println!("Error opening File: {}", e);
                }
            }
        } else {
            println!("Could not find {}", p.to_str().unwrap_or_default());
        }
    }
}
