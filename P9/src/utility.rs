use colored::Colorize;

pub struct Log;

impl Log {
    pub fn hello() {
        println!("{} - poznaj obcych online", "Aero".italic().cyan());
    }

    pub fn info(message: &str) {
        println!("{} - {}", "Info".cyan().bold(), message);
    }

    pub fn warn(message: &str) {
        println!("{} - {}", "Warning".yellow().bold(), message);
    }

    pub fn error(message: &str) {
        println!("{} - {}", "Error".red().bold(), message);
    }
}
