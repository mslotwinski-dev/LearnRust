use std::collections::HashMap;
use std::io::{self, Read, Result, Write};
use std::net::{TcpListener, TcpStream};
use std::sync::mpsc::Sender;
use std::sync::{Arc, Mutex, mpsc};
use std::thread;

use crate::utility::Log;

type Clients = Arc<Mutex<HashMap<String, Sender<String>>>>;

pub fn run_server() -> Result<()> {
    let listener = TcpListener::bind("127.0.0.1:4000")?;
    Log::info(&format!("Listening on http://{}", listener.local_addr()?));

    let clients: Clients = Arc::new(Mutex::new(HashMap::new()));

    while let Ok((stream, _)) = listener.accept() {
        let clients = Arc::clone(&clients);
        thread::spawn(move || {
            handle_client(stream, clients);
        });
    }

    Ok(())
}

fn handle_client(mut stream: TcpStream, clients: Clients) {
    let mut buffer = [0u8; 512];

    let username = match stream.read(&mut buffer) {
        Ok(0) => return,
        Ok(n) => String::from_utf8_lossy(&buffer[..n]).trim().to_string(),
        Err(_) => return,
    };

    Log::info(&format!(
        "User '{}' connected from {}",
        username,
        stream.peer_addr().unwrap()
    ));

    let (tx, rx) = mpsc::channel::<String>();

    {
        let mut clients_guard = clients.lock().unwrap();
        clients_guard.insert(username.clone(), tx);
    }

    let mut write_stream = stream.try_clone().unwrap();
    thread::spawn(move || {
        for msg in rx {
            let _ = write_stream.write_all(msg.as_bytes());
        }
    });

    loop {
        match stream.read(&mut buffer) {
            Ok(0) => {
                Log::warn(&format!("Client {} disconnected", username));

                clients.lock().unwrap().remove(&username);

                break;
            }

            Ok(n) => {
                let msg = String::from_utf8_lossy(&buffer[..n]);
                println!("{}: {}", username, msg);
            }

            Err(ref e) if e.kind() == io::ErrorKind::ConnectionReset => {
                Log::warn(&format!("Client {} disconnected", username));
                clients.lock().unwrap().remove(&username);
                break;
            }

            Err(e) => {
                Log::error(&format!("Error reading from {}: {}", username, e));

                clients.lock().unwrap().remove(&username);

                break;
            }
        }
    }
}
