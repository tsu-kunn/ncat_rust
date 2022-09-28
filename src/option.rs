// オプション情報
pub struct OptionParam {
    opt_n: i8,
    opt_h: i8,
    file_name: String,
}

impl OptionParam {
    pub fn opt_n(&self) -> i8 {
        self.opt_n
    }

    pub fn opt_h(&self) -> i8 {
        self.opt_h
    }

    pub fn file_name(&self) -> String {
        self.file_name.clone()
    }
}
