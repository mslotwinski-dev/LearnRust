use colored::Colorize;
use serde::{Deserialize, Serialize};
use std::fs;
use std::io::{self, Write};
use std::path::Path;

use crate::note::Note;
use crate::utility::Log;

#[derive(Serialize, Deserialize)]
pub struct DB {
    notes: Vec<Note>,
}

fn prompt(message: &str) -> String {
    print!("{}", message);
    io::stdout().flush().unwrap();
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    input.trim().to_string()
}

impl DB {
    pub fn new() -> DB {
        let path = "notes_db.json";

        if Path::new(path).exists() {
            // spróbuj wczytać JSON
            let data = fs::read_to_string(path).unwrap_or_else(|_| {
                Log::warn("Nie udało się odczytać pliku. Tworzę pustą bazę danych.");
                String::new()
            });

            if !data.is_empty()
                && let Ok(db) = serde_json::from_str::<DB>(&data)
            {
                // Log::info("Dane bazy danych wczytane z pliku.");
                return db;
            }
        }

        DB { notes: Vec::new() }
    }

    pub fn add_note(&mut self) {
        let title = prompt("Podaj tytuł notatki: ");

        if title.is_empty() {
            Log::error("Tytuł nie może być pusty.");
            return;
        }

        if self
            .notes
            .iter()
            .any(|note| note.title.to_lowercase() == title.to_lowercase())
        {
            Log::error("Notatka o takim tytule już istnieje.");
            return;
        }

        let content = String::new();

        self.notes.push(Note::new(title, content));

        self.save();
    }

    pub fn print_list_notes(&self) {
        for (i, note) in self.notes.iter().enumerate() {
            println!("{}. {}", i + 1, note.title);
        }
    }

    pub fn open_note(&self, title: String) {
        if let Some(note) = self
            .notes
            .iter()
            .find(|n| n.title.to_lowercase() == title.to_lowercase())
        {
            println!("{}", note.title.bold());
            println!();
            println!("{}", note.content);
        } else {
            Log::error("Nie znaleziono notatki o podanej nazwie.");
        }
    }

    pub fn edit_note(&mut self, title: String) {
        if let Some(note) = self
            .notes
            .iter_mut()
            .find(|n| n.title.to_lowercase() == title.to_lowercase())
        {
            let new_content = prompt("Podaj nową treść notatki: ");
            note.content = new_content;

            self.save();
        } else {
            Log::error("Nie znaleziono notatki o podanej nazwie.");
        }
    }

    pub fn remove_note(&mut self, title: String) {
        if let Some(pos) = self.notes.iter().position(|n| n.title == title) {
            self.notes.remove(pos);
            Log::info(&format!("Notatka '{}' została usunięta.", title));
            self.save();
        } else {
            Log::error("Nie znaleziono notatki o podanej nazwie.");
        }
    }

    fn save(&self) {
        let json = serde_json::to_string_pretty(&self).unwrap();
        std::fs::write("notes_db.json", json)
            .expect("Nie udało się zapisać danych bazy danych do pliku.");
    }
}
