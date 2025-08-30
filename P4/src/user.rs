use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub enum Gender {
    Male,
    Female,
}

#[derive(Serialize, Deserialize)]
pub struct User {
    name: String,
    age: u8,
    email: String,
    gender: Gender,
}

impl User {
    pub fn get_name(&self) -> &str {
        &self.name
    }

    pub fn get_email(&self) -> &str {
        &self.email
    }

    pub fn print_data(&self) {
        println!("Imię i nazwisko: {}", self.get_name());
        println!("Wiek: {}", self.age);
        println!("E-mail: {}", self.get_email());
        println!(
            "Płeć: {}",
            match self.gender {
                Gender::Male => "Mężczyzna",
                Gender::Female => "Kobieta",
            }
        );
    }

    pub fn input_name() -> String {
        let mut name = String::new();
        println!("Jak masz na imię i nazwisko?");

        loop {
            std::io::stdin().read_line(&mut name).unwrap();
            name = name.trim().to_string();
            if name.is_empty() {
                println!("Proszę podać poprawne imię i nazwisko.");
                continue;
            }
            return name;
        }
    }

    pub fn input_age() -> u8 {
        println!("Ile masz lat?");

        loop {
            let mut age_input = String::new();
            std::io::stdin().read_line(&mut age_input).unwrap();

            match age_input.trim().parse::<u8>() {
                Ok(age) => return age,
                Err(_) => {
                    println!("Proszę podać poprawny wiek.");
                    continue;
                }
            }
        }
    }

    pub fn input_email() -> String {
        println!("Podaj swój adres e-mail:");
        loop {
            let mut email = String::new();
            std::io::stdin().read_line(&mut email).unwrap();
            email = email.trim().to_string();
            if email.is_empty() {
                println!("Proszę podać poprawny adres e-mail.");
                continue;
            }
            return email;
        }
    }

    pub fn input_gender() -> Gender {
        println!("Jakiej jesteś płci (M/K)?");

        loop {
            let mut gender_input = String::new();
            std::io::stdin().read_line(&mut gender_input).unwrap();
            match gender_input.trim().to_lowercase().as_str() {
                "m" => return Gender::Male,
                "k" => return Gender::Female,
                _ => {
                    println!("Proszę podać poprawną płeć (M/K).");
                    continue;
                }
            };
        }
    }

    pub fn new() -> Self {
        let name = Self::input_name();
        let age = Self::input_age();
        let email = Self::input_email();
        let gender = Self::input_gender();

        User {
            name,
            age,
            email,
            gender,
        }
    }
}
