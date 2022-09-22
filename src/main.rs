use std::io::Write;
use isatty::stdin_isatty;

fn main() {
    if stdin_isatty() {
        println!("terminal");
    } else {
        let reader = std::io::stdin();
        let mut out = std::io::stdout().lock();

        let mut cnt = 1;

        for line in reader.lines() {
            writeln!(out, "{0: >5}: {1:}", cnt, line.unwrap_or(String::from("Failed to read line"))).unwrap();
            cnt += 1;
        }
    }
}
