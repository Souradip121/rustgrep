use std::fs::File;
use std::io::{BufRead, BufReader};
use std::path::{Path, PathBuf};
use rayon::prelude::*;

use crate::error::AppError;
use crate::matcher::Matcher;

// The result of one matched line
#[derive(Debug)]
pub struct Match {
    pub file: PathBuf,
    pub line_number: usize,
    pub line: String,
}

// Searches one single file
pub fn search_file(
    path: &Path,
    matcher: &dyn Matcher,
    invert: bool,
) -> Result<Vec<Match>, AppError> {
    let file = File::open(path)
        .map_err(|_| AppError::FileNotFound(path.to_path_buf()))?;

    let reader = BufReader::new(file);

    let matches = reader
        .lines()
        .enumerate()
        .filter_map(|(i, line)| {
            let line = line.ok()?;

            if matcher.is_match(&line) ^ invert {
                Some(Match {
                    file: path.to_path_buf(),
                    line_number: i + 1,
                    line,
                })
            } else {
                None
            }
        })
        .collect();

    Ok(matches)
}

// Searches ALL files in parallel using rayon
pub fn search_all(
    files: Vec<PathBuf>,
    matcher: &(dyn Matcher + Sync),
    invert: bool,
) -> Vec<Match> {
    files.par_iter()
        .flat_map(|path| {
            search_file(path, matcher, invert)
                .unwrap_or_default()
        })
        .collect()
}