use crate::generator::{GenerationError, MAX_SIZE, WordGenerator};
use crate::grammar::{Grammar, InvalidNonterminal, InvalidTerminal};
use crate::production::Production;
use crate::symbol::{INIT_SYMBOL, Nonterminal, Symbol, Terminal};
use crate::word::Word;

/**
 *  Generates a random word for a specified grammar.
 */
pub struct RandomWordGenerator<G: Grammar> {
    create_grammar: fn() -> G,
}

impl<G: Grammar> RandomWordGenerator<G> {
    pub fn new(create_grammar: fn() -> G) -> Self {
        return RandomWordGenerator { create_grammar };
    }
}

impl<G: Grammar> WordGenerator for RandomWordGenerator<G> {
    fn generate(&self) -> Result<Word, GenerationError> {
        let mut word = String::new();
        let mut grammar = (self.create_grammar)();
        let mut pending = vec![Symbol::Nonterminal(INIT_SYMBOL)];

        while let Some(symbol) = pending.pop() {
            match symbol {
                Symbol::Terminal(terminal) => {
                    self.handle_max_size(&word)?;
                    self.handle_semantic(terminal, &mut grammar)?;
                    word.push(terminal.id());
                }

                Symbol::Nonterminal(nonterminal) => {
                    let productions = self.get_productions(nonterminal, &grammar)?;
                    pending.extend(productions.symbols().iter().rev().copied());
                }
            };
        }

        return Ok(Word::new(word));
    }
}

impl<G: Grammar> RandomWordGenerator<G> {
    fn handle_max_size(&self, content: &String) -> Result<(), GenerationError> {
        if content.len() >= MAX_SIZE {
            return Err(GenerationError::WordTooLong(MAX_SIZE));
        }

        return Ok(());
    }

    fn handle_semantic(&self, terminal: Terminal, grammar: &mut G) -> Result<(), GenerationError> {
        *grammar = match grammar.handle(terminal) {
            Ok(grammar) => grammar,
            Err(InvalidTerminal(id)) => {
                return Err(GenerationError::InvalidTerminal(id));
            }
        };

        match grammar.semantic(terminal) {
            Ok(semantic) => println!("  → {semantic}"),
            Err(InvalidTerminal(id)) => {
                return Err(GenerationError::InvalidTerminal(id));
            }
        };

        return Ok(());
    }

    fn get_productions(
        &self,
        nonterminal: Nonterminal,
        grammar: &G,
    ) -> Result<Production, GenerationError> {
        let productions = match grammar.productions(nonterminal) {
            Ok(productions) => productions,
            Err(InvalidNonterminal(id)) => {
                return Err(GenerationError::InvalidNonterminal(id));
            }
        };

        let selected_index = rand::random_range(0..productions.len());
        return Ok(productions[selected_index].clone());
    }
}
