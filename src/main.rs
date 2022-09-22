use std::io::Write;
use isatty::stdin_isatty;

macro_rules! line_number_output {
    ($o: ident, $l: expr, $s: expr) => (writeln!($o, "{0: >5}: {1:}", $l, $s).unwrap())
}

fn main() {
    let mut out = std::io::stdout().lock();

    if stdin_isatty() {
        println!("terminal");
    } else {
        let reader = std::io::stdin();
        let mut cnt = 1;

        for line in reader.lines() {
            // writeln!(out, "{0: >5}: {1:}", cnt, line.unwrap_or(String::from("Failed to read line"))).unwrap();
            line_number_output!(out, cnt, line.unwrap_or(String::from("Failed to read line")));
            cnt += 1;
        }
    }
}
