use std::collections::HashMap;

use pulldown_cmark::CodeBlockKind;

#[derive(Debug)]
pub struct CodeBlockInfo {
    pub(crate) language: Option<String>,
    pub(crate) values: HashMap<String, String>,
}

pub enum CodeBlockInfoErrorType {
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
                        .filter_map(|s| s.split_once('='))
                        .map(|(s1, s2)| (s1.to_owned(), s2.to_owned())),
                );
                Ok(Self { language, values })
            }
            _ => Err(CodeBlockInfoErrorType::IndentedCodeBlock),
        }
    }
}
