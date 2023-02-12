#![allow(unused)]

use std::collections::HashMap;
use std::fs::File;
use std::io::Write;
use std::path::Path;

use crate::structs::CodeBlockInfo;
use crate::util::*;

use pulldown_cmark::{CodeBlockKind, CowStr, Event, Parser, Tag};

struct CodeBlock {
    info: Option<CodeBlockInfo>,
    code: String,
}

impl CodeBlock {
    fn new(info: Option<CodeBlockInfo>, code: String) -> Self {
        Self { info, code }
    }
}

pub fn tangle_file<P: AsRef<Path>>(p: P) -> Result<(), ()> {
    let input = read_file(p).unwrap();
    let mut parser = get_parser(&input);
    let code_blocks = get_code_blocks(&mut parser);
    for c in code_blocks {
        if let Some(info) = c.info {
            if let Some(file_name) = info.values.get("file") {
                let mut file = File::create(file_name).map_err(|x| ())?;
                file.write(c.code.as_bytes());
            }
        }
    }
    Ok(())
}

fn get_code_blocks(parser: &mut Parser) -> Vec<CodeBlock> {
    let mut code_blocks = vec![];

    while let Some(e) = parser.next() {
        match e {
            // Inline Code Blocks e.g.: `fn main() {}`
            Event::Code(_) => {
                let code = parser
                    .take_while(|x| !matches!(x, Event::End(Tag::CodeBlock(_))))
                    .filter_map(|x| {
                        if let Event::Text(text) = x {
                            Some(text.to_string())
                        } else {
                            None
                        }
                    })
                    .collect::<String>();
                code_blocks.push(CodeBlock::new(None, code))
            }
            Event::Start(Tag::CodeBlock(CodeBlockKind::Indented)) => {
                let code = parser
                    .take_while(|x| !matches!(x, Event::End(Tag::CodeBlock(_))))
                    .filter_map(|x| {
                        if let Event::Text(text) = x {
                            Some(text.to_string())
                        } else {
                            None
                        }
                    })
                    .collect::<String>();
                code_blocks.push(CodeBlock::new(None, code))
            }
            Event::Start(Tag::CodeBlock(info @ CodeBlockKind::Fenced(_))) => {
                let info = CodeBlockInfo::try_from(info);
                let code = parser
                    .take_while(|x| !matches!(x, Event::End(Tag::CodeBlock(_))))
                    .filter_map(|x| {
                        if let Event::Text(text) = x {
                            Some(text.to_string())
                        } else {
                            None
                        }
                    })
                    .collect::<String>();
                code_blocks.push(CodeBlock::new(info.ok(), code))
            }
            _ => {},
        }
    }
    code_blocks
}
