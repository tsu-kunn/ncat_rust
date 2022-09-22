use isatty::stdin_isatty;

fn main() {
    if stdin_isatty() {
        println!("terminal");
    } else {
        println!("pipeline");
    }
}
