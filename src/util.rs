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
    Parser::new_ext(input, Options::ENABLE_TASKLISTS)
}
