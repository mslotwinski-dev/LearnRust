mod connect;
mod server;
mod utility;

use std::env;

use crate::connect::connect;
use crate::server::run_server;
use crate::utility::Log;

fn main() {
    Log::hello();

    let args: Vec<String> = env::args().collect();

    if args.len() == 1 {
        connect().unwrap();

        return;
    }

    match args[1].as_str() {
        "--help" | "-h" => {
            println!("Dostępne opcje:");
            println!("  aero --help                 Wyświetla tę pomoc");
            println!("  aero --version              Wyświetla wersję programu");
            println!("  aero --start                Uruchamia serwer");
            println!("  aero --connect              Łączy się z serwerem");
        }

        "--version" | "-v" => {
            println!("Aero wersja: {}", env!("CARGO_PKG_VERSION"));
        }

        "--start" | "-s" => {
            run_server().unwrap();
        }

        _ => {
            println!("Nieznany argument. Użyj --help, aby uzyskać pomoc.");
        }
    }
}
