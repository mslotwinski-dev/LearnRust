use colored::{ColoredString, Colorize};

pub fn clear() {
    print!("{}[2J", 27 as char);
}

pub fn bank_name() -> ColoredString {
    "Jak u żyda".italic().yellow()
}
