use pulldown_cmark::{Event, Options, Parser, Tag};
use std::fs::File;
use std::io::Read;
use std::path::Path;

use crate::structs::CodeBlockInfo;

pub fn read_file<P: AsRef<Path>>(path: P) -> Option<String> {
    if (path.as_ref().exists()) {
        let mut contents = String::new();
        File::open(path).unwrap().read_to_string(&mut contents);
        return Some(contents);
    }
    None
}

pub fn get_parser(input: &str) -> Parser {
    Parser::new_ext(input, Options::from(Options::ENABLE_TASKLISTS))
}

fn get_code_blocks<'a>(input: &'a str) -> Box<dyn Iterator<Item = CodeBlockInfo> + 'a> {
    let mut acc = vec![];
    let mut arr = vec![];
    let mut current_info: Option<CodeBlockInfo> = None;
    let mut iter = Parser::new_ext(input, Options::from(Options::ENABLE_TASKLISTS));

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
                            /*Some(mut info) => {
                                info.content = acc
                                    .iter()
                                    .map(|x| match x {
                                        Event::Text(text) => text,
                                        _ => "",
                                    })
                                    .collect();
                                acc = vec![];
                                arr.push(info);
                                current_info = None;
                            }*/
                            _ => {}
                        }
                        continue;
                    }
                    _ => {}
                }
                if in_code_block {
                    acc.push(tag);
                }
            }
            None => break,
        }
    }

    if in_code_block {
        panic!("Unclosed Code Block");
    }

    Box::new(arr.into_iter())
}
