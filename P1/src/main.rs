use rand;
use rand::prelude::IndexedRandom;
use std::io::Write;
use std::io::{self, BufRead};

const GEOGRAFIA: [(&str, &str); 10] = [
    ("Jaka jest stolica Polski?", "Warszawa"),
    ("Jaka jest stolica Niemiec?", "Berlin"),
    ("Jaka jest stolica Norwegii?", "Oslo"),
    ("Jaka jest stolica Szwecji?", "Sztokholm"),
    ("Jaka jest stolica Finlandii?", "Helsinki"),
    ("Jaka jest stolica Danii?", "Kopenhaga"),
    ("Jaka jest stolica Islandii?", "Reykjavik"),
    ("Jaka jest stolica Litwy?", "Wilno"),
    ("Jaka jest stolica Łotwy?", "Ryga"),
    ("Jaka jest stolica Estonii?", "Tallinn"),
];

const HISTORIA: [(&str, &str); 10] = [
    ("Kto był pierwszym prezydentem USA?", "George Washington"),
    ("W którym roku rozpoczęła się II wojna światowa?", "1939"),
    (
        "Kto był królem Anglii podczas bitwy pod Hastings?",
        "Wilhelm Zdobywca",
    ),
    ("W którym roku odbyły się wybory 4 czerwca?", "1989"),
    (
        "Kto był cesarzem Francji podczas rewolucji francuskiej?",
        "Napoleon Bonaparte",
    ),
    ("W którym roku odkryto Amerykę?", "1492"),
    ("Kto był pierwszym premierem III RP?", "Tadeusz Mazowiecki"),
    ("W którym roku zakończyła się I wojna światowa?", "1918"),
    (
        "Kto był liderem ZSRR podczas II wojny światowej?",
        "Józef Stalin",
    ),
    ("W którym roku Polska odzyskała niepodległość?", "1918"),
];

const BIOLOGIA: [(&str, &str); 10] = [
    ("Ile chromosomów ma człowiek?", "46"),
    ("Jaka jest podstawowa jednostka życia?", "Komórka"),
    ("Który organ produkuje insulinę?", "Trzustka"),
    (
        "Jak nazywa się proces fotosyntezy?",
        "Przemiana światła w energię",
    ),
    (
        "Jaki pierwiastek jest niezbędny do oddychania komórkowego?",
        "Tlen",
    ),
    ("Jak nazywa się nauka o roślinach?", "Botanika"),
    ("Jak nazywa się nauka o zwierzętach?", "Zoologia"),
    ("Który organ odpowiada za filtrowanie krwi?", "Nerki"),
    ("Jak nazywa się proces podziału komórki?", "Mitoza"),
    ("Jaki jest największy organ w ludzkim ciele?", "Skóra"),
];

fn main() {
    println!("Witaj w quizie wiedzy ogólnej!");
    println!("Wybierz kategorię");

    loop {
        println!("1. Geografia");
        println!("2. Historia");
        println!("3. Biologia");
        println!("4. Wyjście");
        println!("5. Tablica wyników");

        let mut choice = String::new();
        io::stdin()
            .read_line(&mut choice)
            .expect("Nie udało się odczytać linii");

        match choice.trim() {
            "1" => quiz(GEOGRAFIA),
            "2" => quiz(HISTORIA),
            "3" => quiz(BIOLOGIA),
            "4" => {
                println!("Dziękujemy za grę!");
                break;
            }
            "5" => {
                println!("Tablica wyników:");
                // Tutaj można dodać kod do wyświetlania wyników
                let file = std::fs::OpenOptions::new()
                    .read(true)
                    .open("wyniki.txt")
                    .expect("Nie udało się otworzyć pliku");

                let reader = io::BufReader::new(file);
                for line in reader.lines() {
                    println!("{}", line.expect("Nie udało się odczytać linii"));
                }
            }
            _ => println!("Niepoprawny wybór, spróbuj ponownie."),
        }
    }
}

fn quiz(pytania: [(&str, &str); 10]) {
    let mut score = 0;

    let current_pytania: [(&str, &str); 5] = pytania
        .choose_multiple(&mut rand::rng(), 5)
        .cloned()
        .collect::<Vec<_>>()
        .try_into()
        .expect("Nie udało się wybrać 5 pytań");

    for (pytanie, odpowiedz) in current_pytania {
        println!("Pytanie: {}", pytanie);
        let mut user_input = String::new();

        loop {
            io::stdin()
                .read_line(&mut user_input)
                .expect("Nie udało się odczytać linii");
            if user_input.trim().is_empty() {
                println!("Proszę podać odpowiedź.");
            } else {
                break;
            }
        }

        if odpowiedz == user_input.trim() {
            println!("Dobra odpowiedź! Następne pytanie.");
            score += 1;
        } else {
            println!("Zła odpowiedź! Prawidłowa odpowiedź to: {}", odpowiedz);
        }
    }

    let mut name = String::new();
    println!("Podaj swoje imię do tablicy wyników:");
    io::stdin()
        .read_line(&mut name)
        .expect("Nie udało się odczytać linii");

    let name = name.trim();

    let mut file = std::fs::OpenOptions::new()
        .append(true)
        .create(true)
        .open("wyniki.txt")
        .expect("Nie udało się otworzyć pliku");

    writeln!(file, "{}: {}/{}", name, score, current_pytania.len())
        .expect("Nie udało się zapisać wyniku");

    println!("Twój wynik to: {}/{}", score, current_pytania.len());
}
