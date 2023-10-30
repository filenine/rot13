// A rot13 program that encodes/decodes a string from the command line.
// Started 27 October 2023

use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();
    let input: String = args[1..].join(" ");
    let output = rot13(&input);
    println!("{}", output);
}

fn rot13(input: &str) -> String {
    let uppercase: &str = "ABCDEFGHIJKLMNOPQRSTUVWXYZ";
    let lowercase: String = uppercase.to_ascii_lowercase(); // Quicker and less error-prone than typing it manually

    let mut output: String = String::new();
    
    for c in input.chars() {
        let append_char: char;
        if c.is_ascii_uppercase() {
            let index = uppercase.find(c).expect("This shouldn't be happening!");
            let index = (index + 13) % 26;
            append_char = uppercase[index..].chars().next().unwrap();
        } else if c.is_ascii_lowercase() {
            let index = lowercase.find(c).expect("This shouldn't be happening!");
            let index = (index + 13) % 26;
            append_char = lowercase[index..].chars().next().unwrap();
        } else {
            append_char = c;
        }
        output.push(append_char);
    }

    output
}
