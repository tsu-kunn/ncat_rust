// const COLORS: [&str; 8] = ["\x1b[30m", "\x1b[31m", "\x1b[32m", "\x1b[33m", "\x1b[34m", "\x1b[35m", "\x1b[36m", "\x1b[37m"];

pub enum Colors {
    Black = 0,
    Red,
    Green,
    Yellow,
    Blue,
    Magenta,
    Cyan,
    White,
}

pub fn get_color(color: Colors) -> &'static str {
    match color {
        Colors::Black => "\x1b[30m",
        Colors::Red => "\x1b[31m",
        Colors::Green => "\x1b[32m",
        Colors::Yellow => "\x1b[33m",
        Colors::Blue =>  "\x1b[34m",
        Colors::Magenta => "\x1b[35m",
        Colors::Cyan => "\x1b[36m",
        Colors::White => "\x1b[37m",
    }
}

pub fn get_back_color(color: Colors) -> &'static str {
    match color {
        Colors::Black => "\x1b[40m",
        Colors::Red => "\x1b[41m",
        Colors::Green => "\x1b[42m",
        Colors::Yellow => "\x1b[43m",
        Colors::Blue =>  "\x1b[44m",
        Colors::Magenta => "\x1b[45m",
        Colors::Cyan => "\x1b[46m",
        Colors::White => "\x1b[47m",
    }
}

pub fn error_message(app: &str, msg: String) {
    println!("{}[{} error]{}: {}", get_color(Colors::Red), app, get_color(Colors::White), msg);
}

pub fn warning_message(app: &str, msg: String) {
    println!("{}[{} warning]{}: {}", get_color(Colors::Yellow), app, get_color(Colors::White), msg);
}

// マクロはルート(ncat::)公開となるため use キーワードを使って省略できない
#[macro_export]
macro_rules! line_number_output {
    ($o: ident, $l: expr, $s: expr) => (writeln!($o, "{0:}{1: >5}{2:}: {3:}",
        ncat::console::get_color(ncat::console::Colors::Yellow),
        $l,
        ncat::console::get_color(ncat::console::Colors::White),
        $s).unwrap())
}

