use lettre::transport::smtp::SmtpTransport;
use lettre::transport::smtp::authentication::Credentials;
use lettre::{Message, Transport};
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

    pub fn find_account(&self, username: &str) -> Option<usize> {
        self.accounts
            .iter()
            .position(|a| a.get_username() == username)
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

    pub fn login(&mut self) -> bool {
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

        if let Some(index) = self.find_account(&username) {
            // sprawdzamy hasło na osobnej pożyczce (tylko immutable)
            let valid = {
                let account = &self.accounts[index];
                Bank::validate(account)
            };

            if valid {
                println!("Zalogowano pomyślnie!");
                self.start_session(index);
                true
            } else {
                println!("Niepoprawne hasło!");
                false
            }
        } else {
            println!("Nie znaleziono konta o nazwie '{}'", username);
            false
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

    pub fn start_session(&mut self, index: usize) {
        let account = &mut self.accounts[index];

        println!();
        println!(
            "Witaj ponownie, {}! ({})",
            account.get_username(),
            account.get_id()
        );

        loop {
            println!("Wybierz opcję:");
            println!("1. Pieniądze");
            println!("2. Przelewy");
            println!("3. Dane");
            println!("4. Wyloguj się");

            let mut choice = String::new();
            std::io::stdin().read_line(&mut choice).unwrap();

            match choice.trim() {
                "1" => {
                    let account = &mut self.accounts[index];
                    account.balance();
                }
                "2" => {
                    self.transfer(index);
                }
                "3" => {
                    let account = &mut self.accounts[index];
                    account.data();
                }
                "4" => {
                    println!("Wylogowano pomyślnie!");
                    break;
                }
                _ => println!("Niepoprawny wybór. Spróbuj ponownie."),
            }
        }
    }

    pub fn transfer(&mut self, index: usize) {
        let account = &mut self.accounts[index];
        println!("Twoje saldo wynosi: {}", account.get_cash());

        loop {
            println!("Wybierz opcję:");
            println!("1. Przelew zwykły");
            println!("2. Przelew zagraniczny");
            println!("3. Powrót do menu głównego");

            let mut choice = String::new();
            std::io::stdin().read_line(&mut choice).unwrap();

            match choice.trim() {
                "1" => self.transfer_standard(index),
                "2" => self.transfer_foreign(index),
                "3" => break,
                _ => println!("Niepoprawny wybór. Spróbuj ponownie."),
            }
        }
    }

    fn transfer_standard(&mut self, index: usize) {
        let mut to_input = String::new();
        println!("Podaj nazwę odbiorcy:");
        std::io::stdin().read_line(&mut to_input).unwrap();
        let to_input = to_input.trim();
        let to_index = match self.find_account(to_input) {
            Some(i) => i,
            None => {
                println!("Nie znaleziono konta odbiorcy");
                return;
            }
        };

        if index == to_index {
            println!("Nie możesz przelać samemu sobie");
            return;
        }

        let (first, second) = if index < to_index {
            let (left, right) = self.accounts.split_at_mut(to_index);
            (&mut left[index], &mut right[0])
        } else {
            let (left, right) = self.accounts.split_at_mut(index);
            (&mut right[0], &mut left[to_index])
        };

        let from_account = first;
        let to_account = second;

        let mut amount_input = String::new();
        println!("Podaj kwotę przelewu:");
        std::io::stdin().read_line(&mut amount_input).unwrap();
        let amount: f64 = match amount_input.trim().parse() {
            Ok(a) => a,
            Err(_) => {
                println!("Niepoprawna kwota");
                return;
            }
        };

        if from_account.get_cash() < amount {
            println!("Niewystarczające środki");
            return;
        }

        from_account.add_cash(-amount);
        to_account.add_cash(amount);
        from_account.push_log(
            format!("Przelew do {}", to_account.get_username()).to_string(),
            -amount,
        );
        to_account.push_log(
            format!("Przelew od {}", from_account.get_username()).to_string(),
            amount,
        );

        println!("Przelew wykonany pomyślnie!");
    }

    fn transfer_foreign(&mut self, index: usize) {
        self.transfer_standard(index);
    }

    // fn find_by_tel(&self, phone: &str) -> Option<usize> {
    //     self.accounts.iter().position(|a| a.phone == phone)
    // }

    // fn transfer_blik(&mut self, index: usize) {
    //     let mut to_input = String::new();
    //     println!("Podaj nazwę odbiorcy:");
    //     std::io::stdin().read_line(&mut to_input).unwrap();
    //     let to_input = to_input.trim();
    //     let to_index = match self.find_account(to_input) {
    //         Some(i) => i,
    //         None => {
    //             println!("Nie znaleziono konta odbiorcy");
    //             return;
    //         }
    //     };

    //     if index == to_index {
    //         println!("Nie możesz przelać samemu sobie");
    //         return;
    //     }

    //     let (first, second) = if index < to_index {
    //         let (left, right) = self.accounts.split_at_mut(to_index);
    //         (&mut left[index], &mut right[0])
    //     } else {
    //         let (left, right) = self.accounts.split_at_mut(index);
    //         (&mut right[0], &mut left[to_index])
    //     };

    //     let from_account = first;
    //     let to_account = second;

    //     let mut amount_input = String::new();
    //     println!("Podaj kwotę przelewu:");
    //     std::io::stdin().read_line(&mut amount_input).unwrap();
    //     let amount: f64 = match amount_input.trim().parse() {
    //         Ok(a) => a,
    //         Err(_) => {
    //             println!("Niepoprawna kwota");
    //             return;
    //         }
    //     };

    //     if from_account.get_cash() < amount {
    //         println!("Niewystarczające środki");
    //         return;
    //     }

    //     from_account.add_cash(-amount);
    //     to_account.add_cash(amount);

    //     println!("Przelew wykonany pomyślnie!");
    // }

    pub fn save_json(&self) {
        let json = serde_json::to_string_pretty(&self).unwrap();
        std::fs::write("bank_db.json", json).expect("Nie udało się zapisać danych banku do pliku.");
    }

    pub fn reset_password(&self) {
        let mut user_input = String::new();
        println!("Podaj nazwę użytkownika:");
        std::io::stdin().read_line(&mut user_input).unwrap();
        let user_input = user_input.trim();

        let user_index = match self.find_account(user_input) {
            Some(i) => i,
            None => {
                println!("Nie znaleziono konta");
                return;
            }
        };

        let email = Message::builder()
            .from("Donald Tusk <premier@kprm.gov.pl>".parse().unwrap())
            .to(
                format!("Odbiorca <{}>", self.accounts[user_index].get_email())
                    .parse()
                    .unwrap(),
            )
            .subject("🇩🇪 Oto twoje hasło 🇩🇪")
            .body(format!(
                "Twoje hasło to: {} \n\nPozdrawiam, Donald Tusk.",
                self.accounts[user_index].get_password(),
            ))
            .unwrap();

        let creds = Credentials::new("".to_string(), "".to_string());

        let mailer = SmtpTransport::starttls_relay("smtp.mailgun.org")
            .unwrap()
            .credentials(creds)
            .build();

        match mailer.send(&email) {
            Ok(_) => println!("Mail wysłany!"),
            Err(e) => println!("Błąd przy wysyłaniu: {:?}", e),
        }
    }
}
