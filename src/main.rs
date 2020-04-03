extern crate getopts;
use std::env;
use walkdir::WalkDir;

#[derive(Debug)]
struct Options {
    verbosity: usize,
    threshold: usize,
    limit: usize,
    pattern: String,
    hash_split_threshold: usize,
    dry_run: bool,
    paths: Vec<String>,
}

fn print_usage(program: &str, opts: getopts::Options) {
    let brief = format!("Usage: {} FILE [options]", program);
    print!("{}", opts.usage(&brief));
}

fn parse_command_line(args: Vec<String>) -> Option<Options> {
    let program = args[0].clone();

    let default_pattern = String::from("%m/%z");
    let default_threshold = 1024;
    let default_limit = 0;
    let default_hash_split_threshold = 3;

    let mut opts = getopts::Options::new();
    opts.optopt(
        "s",
        "pattern",
        &format!(
            "Desired organisational structure. (Default: '{}')",
            default_pattern
        ),
        "PATTERN",
    );
    opts.optopt(
        "t",
        "threshold",
        &format!(
            "Maximum files per group in organisation structure. (Default: {})",
            default_threshold
        ),
        "THRESHOLD",
    );
    opts.optflag(
        "",
        "dry-run",
        "Do not modify/move files, only show what would be done.",
    );
    opts.optflag(
        "",
        "disable-group-dir",
        "Do not append an organisational numbered group",
    );
    opts.optflag("", "case-sensitive", "Sort filenames case-sensitively");
    opts.optopt(
        "",
        "limit",
        &format!(
            "Limit number of files to process (default: {})",
            default_limit
        ),
        "LIMIT",
    );
    opts.optflagmulti("v", "", "increase verbosity");
    opts.optflag("h", "help", "print this help menu");
    opts.optopt(
        "",
        "hash-split",
        &format!(
            "Position in SHA1 hash to split path (Default: {})",
            default_hash_split_threshold
        ),
        "THRESHOLD",
    );

    let matches = match opts.parse(&args[1..]) {
        Ok(m) => m,
        Err(f) => panic!(f.to_string()),
    };

    if matches.opt_present("h") {
        print_usage(&program, opts);
        return None;
    }
    let options = Options {
        verbosity: matches.opt_count("v"),
        hash_split_threshold: matches
            .opt_get_default("hash-split", default_hash_split_threshold)
            .unwrap_or(default_hash_split_threshold),
        threshold: matches
            .opt_get_default("t", default_threshold)
            .unwrap_or(default_threshold),
        dry_run: matches.opt_present("dry-run"),
        limit: matches
            .opt_get_default("limit", default_limit)
            .unwrap_or(default_limit),
        pattern: matches
            .opt_get_default("s", default_pattern.clone())
            .unwrap_or(default_pattern),
        paths: matches.free,
    };
    Some(options)
}

fn process(root: &String, entry: &walkdir::DirEntry) -> Option<String> {
    println!("{} -> {:?}", root, entry);
    None
}

fn gather_files(options: Options) {
    for path in options.paths {
        let entries: Vec<_> = WalkDir::new(&path)
            .into_iter()
            .filter_map(|entry| entry.ok().map(|entry| process(&path, &entry)).flatten())
            .collect();
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();

    if let Some(options) = parse_command_line(args) {
        println!("{:?}", options);
        gather_files(options);
    }
}
