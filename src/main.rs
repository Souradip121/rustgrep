mod cli;
mod error;
mod matcher;
mod output;
mod searcher;
mod walker;

fn main() {
    // Step 1 — read what the user typed
    let args = cli::parse();

    // Step 2 — build the right matcher from the pattern + flags
    let matcher = match matcher::build_matcher(
        &args.pattern,
        args.ignore_case,
        args.regex,
    ) {
        Ok(m) => m,
        Err(e) => {
            eprintln!("error: {}", e);
            std::process::exit(1);
        }
    };

    // Step 3 — expand paths into a flat list of files
    let files = match walker::collect_files(&args.paths, args.recursive) {
    Ok(f) => f,
    Err(e) => {
        eprintln!("error: {}", e);
        std::process::exit(1);
    }
};

    // Step 4 — search all files
    let results = searcher::search_all(files, &*matcher, args.invert);

    // Step 5 — print results
    if results.is_empty() {
        std::process::exit(1);  // grep convention — exit 1 if no matches
    }

    output::print_all(&results, args.line_numbers, args.count);
}