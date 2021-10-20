pub enum Mode {
    Normal,
    Insert,
    Replace,
    Visual,
    VisualLine,
    VisualBlock,
    Command,
    Select,
    SelectLine,
    SelectBlock,
    Terminal,
}

impl From<&str> for Mode {
    fn from(mode: &str) -> Self {
        match mode {
            "n" => Mode::Normal,
            "i" => Mode::Insert,
            "R" => Mode::Replace,
            "v" => Mode::Visual,
            "V" => Mode::VisualLine,
            "<C-v>" => Mode::VisualBlock,
            "c" => Mode::Command,
            "s" => Mode::Select,
            "S" => Mode::SelectLine,
            "<C-s>" => Mode::SelectBlock,
            "t" => Mode::Terminal,
            m => {
                eprintln!("unknown mode {}, falling back to Mode::Normal", m);
                Mode::Normal
            }
        }
    }
}
