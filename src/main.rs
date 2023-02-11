#![allow(unused)]

mod printer;
mod structs;
mod util;

use printer::{print_parser_indented, print_parser_raw, print_readme};
use structs::CodeBlockInfo;
use util::{get_parser, read_file};

use std::collections::HashMap;
use std::fmt::{Debug, Display, Formatter, Result as FmtResult};
use std::fs::{File, OpenOptions};
use std::io::{stdout, Read, Write};
use std::path::{Path, PathBuf};

use clap::{ArgAction, Args, Parser as ClapParser, Subcommand};
use pulldown_cmark::{CodeBlockKind, CowStr, Event, Options, Parser, Tag};

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
struct Cli {
    #[command(subcommand)]
    command: CliCommand,
}

#[derive(Debug, Subcommand)]
#[non_exhaustive]
enum CliCommand {
    Tangle(TangleArgs),
    Parse(ParseArgs),
    Format(FormatArgs),
}

#[derive(Debug, Args)]
#[non_exhaustive]
struct TangleArgs {
    #[arg(short, long)]
    path: Vec<PathBuf>,
    #[arg(short, long, default_value_t = false)]
    auto_name: bool,
}

#[derive(Debug, Args)]
#[non_exhaustive]
struct ParseArgs {
    #[arg(short, long)]
    path: PathBuf,
    #[arg(short = 'v', long, default_value_t = true, action = ArgAction::SetFalse)]
    pretty: bool,
    #[arg(short, long, default_value_t = false)]
    show_start_end_tags: bool,
}

#[derive(Debug, Args)]
#[non_exhaustive]
struct FormatArgs {
    #[arg(short, long)]
    in_file: PathBuf,
    #[arg(short, long)]
    out_file: Option<PathBuf>,
}

fn main() {
    let args = Cli::parse();

    match args.command {
        CliCommand::Tangle(TangleArgs {
            path, auto_name, ..
        }) => {
            // TODO
        }
        CliCommand::Parse(ParseArgs {
            path,
            pretty,
            show_start_end_tags,
            ..
        }) => {
            if let Some(contents) = read_file(path) {
                let parser = get_parser(&contents);
                if (pretty) {
                    print_parser_indented(parser, show_start_end_tags);
                } else {
                    print_parser_raw(parser);
                }
            }
        }
        CliCommand::Format(args) => {
            let input = read_file(args.in_file).unwrap();

            match args.out_file {
                Some(path) => match OpenOptions::new()
                    .read(true)
                    .write(true)
                    .create(true)
                    .open(path)
                {
                    Ok(mut f) => write(&input, &mut f),
                    Err(e) => panic!("{}", e),
                },
                None => {
                    let stdout = stdout();
                    write(&input, &mut stdout.lock());
                }
            }
            fn write(input: &str, out: &mut dyn Write) {
                print_readme(get_parser(input), out);
            }
        }
        _ => unimplemented!(),
    }
}
