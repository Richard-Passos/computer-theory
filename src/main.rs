mod dmc_grammar;
mod generator;
mod grammar;
mod production;
mod random_generator;
mod symbol;
mod word;

use std::io::{self, Write};

use crate::dmc_grammar::DMCGrammar;
use crate::generator::{GenerationError, WordGenerator};
use crate::random_generator::RandomWordGenerator;

fn main() {
    let generator = RandomWordGenerator::new(DMCGrammar::new);

    let tries = get_tries();
    for i in 1..=tries {
        println!("Try number {i:02}:");

        match generator.generate() {
            Ok(word) => {
                println!("  → Generated word: {word}");
            }
            Err(GenerationError::WordTooLong(size)) => {
                eprintln!("  → This generation attempt exceeded {size} characters.");
            }
            Err(GenerationError::InvalidTerminal(id)) => {
                eprintln!("  → Invalid terminal '{id}'. A developer must fix it.")
            }
            Err(GenerationError::InvalidNonterminal(id)) => {
                eprintln!("  → Invalid nonterminal '{id}'. A developer must fix it.");
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
        print!("{message}");
    }

    io::stdout()
        .flush()
        .expect("  → Failed to flush stdout. Try again.");
    io::stdin()
        .read_line(&mut input)
        .expect("  → Failed to read input. Try again.");

    return input;
}
