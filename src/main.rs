#![allow(unused)]

mod util;
mod printer;
mod structs;

use util::{get_parser, read_file};
use printer::{print_parser_indented, print_parser_raw};
use structs::CodeBlockInfo;

use std::collections::HashMap;
use std::fmt::{Debug, Display, Formatter, Result as FmtResult};
use std::fs::File;
use std::io::{Read, Write};
use std::path::{Path, PathBuf};

use clap::{Args, Parser as ClapParser, Subcommand, ArgAction};
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
	show_start_end_tags: bool
}

fn main() {
    let args = Cli::parse();

    match args.command {
        CliCommand::Tangle(TangleArgs { path, auto_name, .. }) => {
			// TODO
		}
        CliCommand::Parse(ParseArgs { path, pretty, show_start_end_tags, .. }) => {
            if let Some(contents) = read_file(path) {
				let parser = get_parser(&contents);
				if (pretty) {
                	print_parser_indented(parser, show_start_end_tags);
				} else {
					print_parser_raw(parser);
				}
            }
        }
    }
}
