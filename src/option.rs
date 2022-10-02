// オプション情報
pub struct OptionParam {
    opt_n: bool,
    opt_h: bool,
    file_name: String,
    args: Vec<String>,
}

impl OptionParam {
    pub fn new() -> Self {
        Self {
            opt_n: false,
            opt_h: false,
            file_name: String::new(),
            args: std::env::args().collect(),
        }
    }

    pub fn opt_n(&self) -> bool {
        self.opt_n
    }

    pub fn opt_h(&self) -> bool {
        self.opt_h
    }

    pub fn file_name(&self) -> String {
        self.file_name.clone()
    }

    pub fn info_draw(&self) {
        println!("ncat [option] [in file] ...");
        println!("    [option]");
        println!("       -n : 行番号なし");
        println!("       -h : シンタックスハイライトなし");
        println!("       -? : ヘルプ出力");
        print!("\n");
    }

    pub fn check_args(&self) -> bool {
        // 最低限必要な引数の数があるかをチェックする
        if self.args.len() < 2 {
            return false;
        }
        return true;
    }
}
