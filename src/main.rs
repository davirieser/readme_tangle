#![allow(unused)]

use std::fmt::{Display, Debug, Formatter, Result};
use clap::Parser;

#[derive(Parser, Debug)]
#[command(about = "Extract Source Code Blocks from a README", version, long_about = None)]
struct Args {
}

fn main() {
    let args = Args::parse();

}
