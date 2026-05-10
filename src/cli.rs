use std::path::PathBuf;
use clap::Parser;

#[derive(Parser, Debug)]
#[command(
    name = "rustgrep",
    version = "1.0",
    about = "A fast grep clone written in Rust"
)]
pub struct Args {
    /// Pattern to search for
    pub pattern: String,

    /// Files or directories to search
    pub paths: Vec<PathBuf>,

    /// Ignore case distinctions
    #[arg(short = 'i', long)]
    pub ignore_case: bool,

    /// Search directories recursively
    #[arg(short = 'r', long)]
    pub recursive: bool,

    /// Show line numbers
    #[arg(short = 'n', long)]
    pub line_numbers: bool,

    /// Only print count of matching lines
    #[arg(short = 'c', long)]
    pub count: bool,

    /// Invert match — show lines that do NOT match
    #[arg(short = 'v', long)]
    pub invert: bool,

    /// Use full regex matching
    #[arg(short = 'E', long)]
    pub regex: bool,
}

pub fn parse() -> Args {
    Args::parse()
}