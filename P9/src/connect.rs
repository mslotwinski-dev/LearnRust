use std::io::{self, Read, Write};
use std::net::TcpStream;
use std::thread;

pub fn connect() -> io::Result<()> {
    let username = get_username();

    let mut stream = TcpStream::connect("127.0.0.1:4000")?;
    println!("Connected to server!");

    stream.write_all(username.as_bytes())?;

    let mut read_stream = stream.try_clone()?;

    thread::spawn(move || {
        let mut buffer = [0u8; 512];
        loop {
            match read_stream.read(&mut buffer) {
                Ok(0) => {
                    println!("Server disconnected");
                    return;
                }
                Ok(n) => {
                    let msg = String::from_utf8_lossy(&buffer[..n]);
                    println!("From server: {}", msg);
                }
                Err(e) => {
                    eprintln!("Error reading from server: {}", e);
                    return;
                }
            }
        }
    });

    loop {
        let mut input = String::new();
        io::stdin().read_line(&mut input)?;
        stream.write_all(input.trim().as_bytes())?;
    }
}

fn get_username() -> String {
    use std::io::{self, Write};

    print!("Enter your username: ");
    io::stdout().flush().unwrap();

    let mut username = String::new();
    io::stdin().read_line(&mut username).unwrap();
    username.trim().to_string()
}
