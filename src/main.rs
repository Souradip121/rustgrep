mod cli;
mod matcher;
mod searcher;
mod walker;
mod output;
mod error;

use error::AppError;   // ← add this

fn main() {
    println!("rustgrep");
}