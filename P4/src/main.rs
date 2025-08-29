use crate::bank::Bank;
use crate::utility::{bank_name, clear};

mod account;
mod bank;
mod user;
mod utility;

fn get_user_input() -> String {
    let mut input = String::new();
    std::io::stdin().read_line(&mut input).unwrap();
    input.trim().to_string()
}

fn main() {
    clear();
    println!("Witamy w banku {}!", bank_name());

    let mut bank = Bank::new();

    loop {
        println!();
        println!("Wybierz opcję z oferty: ");
        println!("1. Logowanie lub rejestracja");
        println!("2. Przypomnienie hasła");
        println!("3. Kontakt");

        let choice = get_user_input();

        match choice.as_str() {
            "1" => {
                if let Some(account) = bank.login() {
                    account.start_session();
                } else {
                    println!("Logowanie nieudane.");
                }
            }
            "2" => {
                // bank.reset_password();
            }
            "3" => {
                bank.contact_support();
            }
            _ => {
                println!("Nieznana opcja.");
            }
        }
    }
}
