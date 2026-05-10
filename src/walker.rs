use std::path::PathBuf;
use walkdir::WalkDir;
use crate::error::AppError;

pub fn collect_files(
    paths: &[PathBuf],
    recursive: bool,
) -> Result<Vec<PathBuf>, AppError> {
    let mut result = Vec::new();

    for path in paths {
        if path.is_dir() {
            if recursive {
                WalkDir::new(path)
                    .into_iter()
                    .filter_map(|e| e.ok())
                    .filter(|e| e.file_type().is_file())
                    .for_each(|e| result.push(e.into_path()));
            } else {
                eprintln!("warning: {} is a directory, use -r", path.display());
            }
        } else if path.exists() {
            result.push(path.clone());
        } else {
            return Err(AppError::FileNotFound(path.clone()));
        }
    }

    Ok(result)
}