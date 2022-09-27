use std::io::{Write, BufRead};
use std::fs::File;
use isatty::stdin_isatty;

use ncat::console;


fn main() {
    let mut cnt = 1;
    let mut out = std::io::stdout().lock();

    // let err_msg = format!("{}[ncat error]{}: ", get_color(ConsoleColor::Red), get_color(ConsoleColor::White));

    if stdin_isatty() {
        let args: Vec<String> = std::env::args().collect();

        println!("{:?}, {}", args, args.len());

        if args.len() < 2 { std::process::exit(1) }

        let fname = &args[1];
        let f = File::open(fname);
        let f = match f {
            Ok(file) => file,
            Err(error) => {
                console::ncat_errmsg(error.to_string());
                std::process::exit(1);
            }
        };

        let reader = std::io::BufReader::new(f);

        for line in reader.lines() {
            ncat::line_number_output!(out, cnt, line.unwrap_or(String::from("Failed to read line.")));
            cnt += 1;
        }
    } else {
        let reader = std::io::stdin();

        for line in reader.lines() {
            // writeln!(out, "{0: >5}: {1:}", cnt, line.unwrap_or(String::from("Failed to read line."))).unwrap();
            ncat::line_number_output!(out, cnt, line.unwrap_or(String::from("Failed to read line.")));
            cnt += 1;
        }
    }

    std::process::exit(0);
}
