use owo_colors::OwoColorize;
use crate::searcher::Match;

pub fn print_match(
    m: &Match,
    show_line_numbers: bool,
    multiple_files: bool,
) {
    // build the output line piece by piece
    let mut output = String::new();

    // if searching multiple files, prefix with filename
    if multiple_files {
        output.push_str(&format!("{}:", m.file.display().to_string().cyan()));
    }

    // if -n flag set, add line number
    if show_line_numbers {
        output.push_str(&format!("{}:", m.line_number.to_string().green()));
    }

    // always print the line itself
    output.push_str(&m.line);

    println!("{}", output);
}

pub fn print_all(
    matches: &[Match],
    show_line_numbers: bool,
    count_only: bool,
) {
    // if -c flag, just print the count and stop
    if count_only {
        println!("{}", matches.len());
        return;
    }

    let multiple_files = {
        let mut files = matches.iter().map(|m| &m.file);
        let first = files.next();
        files.any(|f| Some(f) != first)
    };

    for m in matches {
        print_match(m, show_line_numbers, multiple_files);
    }
}