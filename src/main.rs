mod dmc_grammar;
mod generator;
mod grammar;
mod production;
mod symbol;
mod word;

use std::io::{self, Write};

use crate::dmc_grammar::DMCGrammar;
use crate::generator::{GenerationError, MAX_SIZE, RandomWordGenerator, WordGenerator};

fn main() {
    let grammar = DMCGrammar::new();
    let generator = RandomWordGenerator::new(&grammar);

    let tries = get_tries();
    for i in 1..=tries {
        println!("Try number {:02}:", i);

        match generator.generate() {
            Ok(word) => {
                println!("  → Generated word: {}", word.content());
            }
            Err(GenerationError::WordTooLong) => {
                eprintln!("  → This generation attempt exceeded {MAX_SIZE} characters.");
            }
            Err(GenerationError::InvalidTerminal) => {
                eprintln!("  → The word contains an invalid terminal. A developer must fix it.")
            }
            Err(GenerationError::InvalidNonterminal) => {
                eprintln!("  → The word contains an invalid nonterminal. A developer must fix it.");
            }
        };

        println!();
    }
}

fn get_tries() -> usize {
    return loop {
        let input = input("Enter the number of tries to generate the word: ");

        match input.trim().parse::<usize>() {
            Ok(tries) => break tries,
            Err(_) => eprintln!("  → Invalid value! Enter a non-negative integer.\n"),
        };
    };
}

fn input(message: &str) -> String {
    let mut input = String::new();

    if !message.is_empty() {
        print!("{}", message);
    }

    io::stdout()
        .flush()
        .expect("  → Failed to flush stdout. Try again.");
    io::stdin()
        .read_line(&mut input)
        .expect("  → Failed to read input. Try again.");

    return input;
}
