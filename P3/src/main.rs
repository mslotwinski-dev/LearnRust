// use std::fs;
use std::io;

fn main() {
    println!("Szyfr Cezara");

    // let content = fs::read_to_string("key.txt").expect("Failed to read file");
    // let key: &str = content.trim();

    const KEY: u8 = 3;

    let mut input = String::new();
    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read line");

    println!("You entered: {}", input.trim());

    let mut encrypted = String::new();

    for c in input.chars() {
        let encrypted_char = caesar_char(c, KEY);
        encrypted.push(encrypted_char);
    }

    println!("Encrypted: {}", encrypted);
}

fn caesar_char(c: char, shift: u8) -> char {
    if c.is_ascii_uppercase() {
        let base = b'A';
        let pos = c as u8 - base;
        let new_pos = (pos + shift) % 26;
        (base + new_pos) as char
    } else if c.is_ascii_lowercase() {
        let base = b'a';
        let pos = c as u8 - base;
        let new_pos = (pos + shift) % 26;
        (base + new_pos) as char
    } else if c.is_ascii_digit() {
        let base = b'0';
        let pos = c as u8 - base;
        let new_pos = (pos + shift) % 10;
        (base + new_pos) as char
    } else {
        c
    }
}
