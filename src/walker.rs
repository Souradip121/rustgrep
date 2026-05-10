use std::path::PathBuf;
use walkdir::WalkDir;

pub fn collect_files(paths: &[PathBuf], recursive: bool) -> Vec<PathBuf> {
    paths.iter().flat_map(|path| {
        if path.is_dir() {
            if recursive {
                // walk the whole directory tree
                WalkDir::new(path)
                    .into_iter()
                    .filter_map(|entry| entry.ok())
                    .filter(|entry| entry.file_type().is_file())
                    .map(|entry| entry.into_path())
                    .collect::<Vec<_>>()
            } else {
                // directory given but -r not set — skip it
                eprintln!("warning: {} is a directory, use -r to search recursively", path.display());
                vec![]
            }
        } else {
            // it's already a file — just use it directly
            vec![path.clone()]
        }
    }).collect()
}