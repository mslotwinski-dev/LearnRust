use serde::{Deserialize, Serialize};

use rand::Rng;

use crate::user::User;

#[derive(Serialize, Deserialize)]
pub struct Account {
    id: u32,
    user: User,
    username: String,
    password: String,
    money: f64,
    logs: Vec<Log>,
}

#[derive(Serialize, Deserialize)]
struct Log {
    action: String,
    amount: f64,
}

impl Log {
    pub fn new(action: String, amount: f64) -> Self {
        Log { action, amount }
    }
}

impl Account {
    pub fn get_username(&self) -> String {
        self.username.clone()
    }

    pub fn get_email(&self) -> &str {
        self.user.get_email()
    }

    pub fn get_id(&self) -> u32 {
        self.id
    }

    pub fn get_cash(&self) -> f64 {
        self.money
    }

    pub fn add_cash(&mut self, amount: f64) {
        self.money += amount;
    }

    pub fn get_password(&self) -> &str {
        &self.password
    }

    pub fn push_log(&mut self, action: String, amount: f64) {
        self.logs.push(Log::new(action, amount));
    }

    fn input_username() -> String {
        let mut username = String::new();

        println!("Podaj nazwę użytkownika:");
        loop {
            std::io::stdin().read_line(&mut username).unwrap();
            username = username.trim().to_string();
            if username.is_empty() {
                println!("Proszę podać poprawną nazwę użytkownika.");
                continue;
            }
            return username;
        }
    }

    fn input_password() -> String {
        let mut password = String::new();

        println!(
            "Podaj hasło (poprawne hasło ma co najmniej 8 znaków, w tym jedną wielką literę, jedną małą literę i jedną cyfrę):"
        );
        loop {
            std::io::stdin().read_line(&mut password).unwrap();
            password = password.trim().to_string();
            if password.is_empty() {
                println!("Proszę podać poprawne hasło.");
                continue;
            }

            if password.len() < 8 {
                println!("Hasło musi mieć co najmniej 8 znaków.");
                continue;
            }

            if !password.chars().any(|c| c.is_lowercase()) {
                println!("Hasło musi zawierać co najmniej jedną małą literę.");
                continue;
            }

            if !password.chars().any(|c| c.is_uppercase()) {
                println!("Hasło musi zawierać co najmniej jedną wielką literę.");
                continue;
            }

            if !password.chars().any(|c| c.is_digit(10)) {
                println!("Hasło musi zawierać co najmniej jedną cyfrę.");
                continue;
            }

            return password;
        }
    }

    fn generate_id() -> u32 {
        let mut rng = rand::rng();
        rng.random_range(10000000..=99999999)
    }

    pub fn check_password(&self, password: &str) -> bool {
        self.password == password
    }

    pub fn balance(&mut self) {
        println!("Twoje saldo wynosi: {}", self.money);

        loop {
            println!("Wybierz opcję:");
            println!("1. Wpłać pieniądze");
            println!("2. Wypłać pieniądze");
            println!("3. Powrót do menu głównego");

            let mut choice = String::new();
            std::io::stdin().read_line(&mut choice).unwrap();

            match choice.trim() {
                "1" => self.deposit(),
                "2" => self.withdraw(),
                "3" => {
                    break;
                }
                _ => println!("Niepoprawny wybór. Spróbuj ponownie."),
            }
        }
    }

    fn deposit(&mut self) {
        println!("Wybierz ilość pieniędzy do wpłacenia:");
        let mut amount = String::new();
        std::io::stdin().read_line(&mut amount).unwrap();
        let amount: f64 = amount.trim().parse().unwrap();
        self.money += amount;
        println!("Wpłacono: {}", amount);
        self.push_log("Wpłata".to_string(), amount);
    }

    fn withdraw(&mut self) {
        println!("Wybierz ilość pieniędzy do wypłacenia:");
        let mut amount = String::new();
        std::io::stdin().read_line(&mut amount).unwrap();
        let amount: f64 = amount.trim().parse().unwrap();
        if amount > self.money {
            println!("Nie masz wystarczająco dużo pieniędzy.");
            return;
        }
        self.money -= amount;
        println!("Wypłacono: {}", amount);
        self.push_log("Wypłata".to_string(), -amount);
    }

    pub fn data(&mut self) {
        loop {
            println!("Wybierz opcję:");
            println!("1. Historia przelewów");
            println!("2. Moje dane");
            println!("3. Powrót do menu głównego");

            let mut choice = String::new();
            std::io::stdin().read_line(&mut choice).unwrap();

            match choice.trim() {
                "1" => self.history(),
                "2" => self.user.print_data(),
                "3" => {
                    break;
                }
                _ => println!("Niepoprawny wybór. Spróbuj ponownie."),
            }
        }
    }

    fn history(&mut self) {
        println!("Historia transakcji:");

        println!("{:<50} {:<10}", "Akcja", "Kwota");
        println!("{}", "-".repeat(65));

        for log in &self.logs {
            println!("{:<50} {:<10}", log.action, log.amount);
        }
    }

    pub fn new(user: User) -> Self {
        let id = Self::generate_id();
        let username = Self::input_username();
        let password = Self::input_password();

        println!("Dziękujemy! Twoje dane są poprawne.");
        println!("Twój numer konta to: {}", id);

        Account {
            id,
            user,
            username,
            password,
            money: 0.0,
            logs: Vec::new(),
        }
    }
}
