use std::io::{Write, BufRead};
use std::fs::File;
use isatty::stdin_isatty;

const COLORS: [&str; 8] = ["\x1b[30m", "\x1b[31m", "\x1b[32m", "\x1b[33m", "\x1b[34m", "\x1b[35m", "\x1b[36m", "\x1b[37m"];

enum ConsoleColor {
    Black = 0,
    Red,
    Green,
    Yellow,
    Blue,
    Magenta,
    Cyan,
    White,
}

macro_rules! line_number_output {
    ($o: ident, $l: expr, $s: expr) => (writeln!($o, "{0: >5}: {1:}", $l, $s).unwrap())
}

fn get_color(color: ConsoleColor) -> &'static str {
    match color {
        ConsoleColor::Black => "\x1b[30m",
        ConsoleColor::Red => "\x1b[31m",
        ConsoleColor::Green => "\x1b[32m",
        ConsoleColor::Yellow => "\x1b[33m",
        ConsoleColor::Blue =>  "\x1b[34m",
        ConsoleColor::Magenta => "\x1b[35m",
        ConsoleColor::Cyan => "\x1b[36m",
        ConsoleColor::White => "\x1b[37m",
    }
}

fn get_back_color(color: ConsoleColor) -> &'static str {
    match color {
        ConsoleColor::Black => "\x1b[40m",
        ConsoleColor::Red => "\x1b[41m",
        ConsoleColor::Green => "\x1b[42m",
        ConsoleColor::Yellow => "\x1b[43m",
        ConsoleColor::Blue =>  "\x1b[44m",
        ConsoleColor::Magenta => "\x1b[45m",
        ConsoleColor::Cyan => "\x1b[46m",
        ConsoleColor::White => "\x1b[47m",
    }
}

fn ncat_errmsg(msg: String) {
    println!("{}[ncat error]{}: {}", get_color(ConsoleColor::Red), get_color(ConsoleColor::White), msg);
}

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
                ncat_errmsg(error.to_string());
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
