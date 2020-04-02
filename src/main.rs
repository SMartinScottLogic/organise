extern crate getopts;
use getopts::Options;
use std::env;

fn do_work(inp: &str, out: Option<String>) {
    println!("{}", inp);
    match out {
        Some(x) => println!("{}", x),
        None => println!("No Output"),
    }
}

fn print_usage(program: &str, opts: Options) {
    let brief = format!("Usage: {} FILE [options]", program);
    print!("{}", opts.usage(&brief));
}

fn parse_command_line(args: Vec<String>) {
    let program = args[0].clone();

    let threshold = 1024;
    let limit = 0;
    let pattern = String::from("%m/%z");
    let hash_split_threshold = 3;

    let mut opts = Options::new();
    opts.optopt("s", "pattern", &format!("Desired organisational structure. (Default: '{}')", pattern), "PATTERN");
    opts.optopt("t", "threshold", &format!("Maximum files per group in organisation structure. (Default: {})", &threshold), "THRESHOLD");
    opts.optflag("", "dry-run", "Do not modify/move files, only show what would be done.");
    opts.optflag("", "disable-group-dir", "Do not append an organisational numbered group");
    opts.optflag("", "case-sensitive", "Sort filenames case-sensitively");
    opts.optopt("", "limit", &format!("Limit number of files to process (default: {})", &limit), "LIMIT");
    opts.optflagmulti("v", "", "increase verbosity");
    opts.optflag("h", "help", "print this help menu");
    opts.optopt("1", "hash-split", &format!("Position in SHA1 hash to split path (Default: {})", hash_split_threshold), "THRESHOLD");

    let matches = match opts.parse(&args[1..]) {
        Ok(m) => { m }
        Err(f) => { panic!(f.to_string()) }
    };

    println!("verbosity: {}", matches.opt_count("v"));
    if matches.opt_present("h") {
        print_usage(&program, opts);
        return;
    }
    /*
    let output = matches.opt_str("o");
    let input = if !matches.free.is_empty() {
        matches.free[0].clone()
    } else {
        print_usage(&program, opts);
        return;
    };
    do_work(&input, output);
    */
}

fn main() {
    let args: Vec<String> = env::args().collect();

    parse_command_line(args);
}
