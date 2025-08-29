use serde::{Deserialize, Serialize};

use std::fs;
use std::path::Path;

use crate::account::Account;
use crate::user::User;
use crate::utility::{bank_name, clear};

#[derive(Serialize, Deserialize)]
pub struct Bank {
    accounts: Vec<Account>,
}

impl Bank {
    pub fn new() -> Self {
        let path = "bank_db.json";

        if Path::new(path).exists() {
            // spróbuj wczytać JSON
            let data = fs::read_to_string(path).unwrap_or_else(|_| {
                println!("Nie udało się odczytać pliku. Tworzę pusty bank.");
                String::new()
            });

            if !data.is_empty()
                && let Ok(bank) = serde_json::from_str::<Bank>(&data)
            {
                println!("Dane banku wczytane z pliku.");
                return bank;
            }
        }

        Bank {
            accounts: Vec::new(),
        }
    }

    pub fn create_account(&mut self, account: Account) -> &Account {
        self.accounts.push(account);
        self.accounts.last().unwrap()
    }

    pub fn find_account(&mut self, username: &str) -> Option<&mut Account> {
        self.accounts
            .iter_mut()
            .find(|account| account.get_username() == username)
    }

    pub fn contact_support(&self) {
        println!();
        println!("Skontaktuj się z naszym zespołem.\n");

        println!(
            "{:<40} {:<40} {:<35} {:<15}",
            "Stanowisko", "Imię i nazwisko", "Email", "Telefon"
        );
        println!("{}", "-".repeat(135));

        println!(
            "{:<40} {:<40} {:<35} {:<15}",
            "Prezes",
            "prof. Janusz Wincent Żalno-Spaślak",
            "janusz.zalno-spaslak@bank.pl",
            "+48 600 100 200"
        );

        println!(
            "{:<40} {:<40} {:<35} {:<15}",
            "Dyrektor ds. badań i rozwoju",
            "inż. Mateusz Słotwiński",
            "mateusz.slotwinski@bank.pl",
            "+48 600 100 212"
        );

        println!(
            "{:<40} {:<40} {:<35} {:<15}",
            "Dyrektor ds. operacyjnych",
            "mgr. Jan Soplica",
            "jan.soplica@bank.pl",
            "+48 600 100 244"
        );

        println!(
            "{:<40} {:<40} {:<35} {:<15}",
            "Zespół wsparcia technicznego",
            "Fredi Kamionka Gmina-Burzenin",
            "fredi.gmina-burzenin@bank.pl",
            "+48 600 100 239"
        );

        println!(
            "{:<40} {:<40} {:<35} {:<15}",
            "Dział logistyki i zakupów",
            "Adrian Nowak-Żalno",
            "adrian.nowak-zalno@bank.pl",
            "+48 600 100 254"
        );

        println!(
            "{:<40} {:<40} {:<35} {:<15}",
            "Dział kontaktów międzynarodowych",
            "mgr. Mania Kulka",
            "mania.kulka@bank.pl",
            "+48 600 100 275"
        );
    }

    pub fn login(&mut self) -> Option<&mut Account> {
        println!();
        println!("Wpisz swoją nazwę użytkownika (zostaw puste pole aby się zarejestrować): ");
        let mut username = String::new();
        std::io::stdin().read_line(&mut username).unwrap();
        let username = username.trim();

        let username = if username.is_empty() {
            self.register()
        } else {
            username.to_string()
        };

        match self.find_account(&username) {
            Some(account) => {
                if Bank::validate(account) {
                    println!("Zalogowano pomyślnie!");
                    Some(account)
                } else {
                    println!("Niepoprawne hasło!");
                    None
                }
            }
            None => {
                println!("Nie znaleziono konta o nazwie '{}'", username);
                None
            }
        }
    }

    fn validate(account: &Account) -> bool {
        println!("Podaj hasło: ");
        let mut password = String::new();
        std::io::stdin().read_line(&mut password).unwrap();
        let password = password.trim();

        account.check_password(password)
    }

    fn register(&mut self) -> String {
        clear();
        println!("Witamy w banku {}!", bank_name());
        println!("Dziękujemy za wybranie naszej oferty!");
        println!("Na początek zadamy ci kilka pytań:");

        let user = User::new();

        println!("Teraz możesz się zarejestrować.");

        let account = self.create_account(Account::new(user));

        println!("Rejestracja zakończona sukcesem! Teraz możesz się zalogować!");

        return account.get_username();
    }

    pub fn save_json(&self) {
        let json = serde_json::to_string_pretty(&self).unwrap();
        std::fs::write("bank_db.json", json).expect("Nie udało się zapisać danych banku do pliku.");
    }
}
