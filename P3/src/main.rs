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
    const LOWERCASE: &str = "aąbcćdeęfghijklłmnńoópqrśstuvwxyzźż";
    const UPPERCASE: &str = "AĄBCĆDEĘFGHIJKLŁMNŃOÓPQRŚSTUVWXYZŹŻ";
    const DIGITS: &str = "0123456789";

    if c.is_ascii_uppercase() {
        let chars = UPPERCASE.chars().collect::<Vec<_>>();
        if let Some(index) = chars.iter().position(|&x| x == c) {
            let new_index = (index as u8 + shift) % chars.len() as u8;
            chars[new_index as usize]
        } else {
            c
        }
    } else if c.is_ascii_lowercase() {
        let chars = LOWERCASE.chars().collect::<Vec<_>>();
        if let Some(index) = chars.iter().position(|&x| x == c) {
            let new_index = (index as u8 + shift) % chars.len() as u8;
            chars[new_index as usize]
        } else {
            c
        }
    } else if c.is_ascii_digit() {
        let chars = DIGITS.chars().collect::<Vec<_>>();
        if let Some(index) = chars.iter().position(|&x| x == c) {
            let new_index = (index as u8 + shift) % chars.len() as u8;
            chars[new_index as usize]
        } else {
            c
        }
    } else {
        c
    }
}
