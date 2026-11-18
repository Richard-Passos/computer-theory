mod generator;
mod grammar;
mod production;
mod symbol;
mod word;

use std::io;

use crate::generator::{GenerationError, MAX_SIZE, RandomWordGenerator, WordGenerator};
use crate::grammar::DMCGrammar;

fn main() {
    let grammar = DMCGrammar::new();
    let generator = RandomWordGenerator::new(&grammar);

    let mut tries = get_tries();
    while tries > 0 {
        match generator.generate() {
            Ok(word) => {
                print!("Generated word: {} -- ", word.content());
            }
            Err(GenerationError::WordTooLong) => {
                eprintln!("This generation attempt exceeded {MAX_SIZE} characters.");
            }
            Err(GenerationError::InvalidNonTerminal) => {
                eprintln!("The grammar contains an invalid non terminal.");
            }
        };

        tries -= 1;
    }
}

fn get_tries() -> usize {
    return loop {
        let mut input = String::new();

        println!("Enter the number of tries to generate the word.");
        io::stdin()
            .read_line(&mut input)
            .expect("Failed to read input");

        match input.trim().parse::<usize>() {
            Ok(size) => break size,
            Err(_) => eprintln!("Invalid value! Enter a non-negative integer."),
        }
    };
}
