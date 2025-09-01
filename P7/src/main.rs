mod db;
mod note;
mod utility;

use std::env;

use crate::db::DB;
use crate::utility::Log;

fn main() {
    Log::hello();

    let mut db = DB::new();
    let args: Vec<String> = env::args().collect();

    if args.len() == 1 {
        println!("Wersja: {}", env!("CARGO_PKG_VERSION"));
        println!("Brak argumentów. Użyj --help, aby uzyskać pomoc.");
        return;
    }

    match args[1].as_str() {
        "--help" | "-h" => {
            println!("Dostępne opcje:");
            println!("  pixel --help                Wyświetla tę pomoc");
            println!("  pixel --list                Wyświetla listę notatek");
            println!("  pixel --add                 Dodaje nową notatkę");
            println!("  pixel --edit [tytuł]        Edytuje treść notatki");
            println!("  pixel --remove [tytuł]      Usuwa notatkę");
            println!("  pixel [tytuł]               Otwiera notatkę");
        }
        "--list" | "-l" => db.print_list_notes(),
        "--add" | "-a" => db.add_note(),
        "--edit" | "-e" => {
            if args.len() < 3 {
                eprintln!("Brak tytułu notatki. Użycie: pixel --edit [tytuł]");
                return;
            }
            db.edit_note(args[2].clone());
        }
        "--remove" | "-r" => {
            if args.len() < 3 {
                eprintln!("Brak tytułu notatki. Użycie: pixel --remove [tytuł]");
                return;
            }
            db.remove_note(args[2].clone());
        }
        note_title => db.open_note(note_title.to_string()),
    }
}
