use std::io::{Write, BufRead};
use std::fs::File;
use isatty::stdin_isatty;

macro_rules! line_number_output {
    ($o: ident, $l: expr, $s: expr) => (writeln!($o, "{0: >5}: {1:}", $l, $s).unwrap())
}

fn main() {
    let mut cnt = 1;
    let mut out = std::io::stdout().lock();

    if stdin_isatty() {
        println!("terminal");

        let args: Vec<String> = std::env::args().collect();

        println!("{:?}, {}", args, args.len());

        if args.len() < 2 { std::process::exit(1) }

        let fname = &args[1];
        let f = File::open(fname);
        let f = match f {
            Ok(file) => file,
            Err(error) => {
                println!("[ncat error]: {}", error);
                std::process::exit(1);
            }
        };

        let reader = std::io::BufReader::new(f);

        for line in reader.lines() {
            line_number_output!(out, cnt, line.unwrap_or(String::from("Failed to read line.")));
            cnt += 1;
        }
    } else {
        let reader = std::io::stdin();

        for line in reader.lines() {
            // writeln!(out, "{0: >5}: {1:}", cnt, line.unwrap_or(String::from("Failed to read line."))).unwrap();
            line_number_output!(out, cnt, line.unwrap_or(String::from("Failed to read line.")));
            cnt += 1;
        }
    }

    std::process::exit(0);
}
