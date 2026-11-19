use crate::grammar::{Grammar, InvalidNonterminal, InvalidTerminal};
use crate::symbol::{INIT_SYMBOL, Nonterminal, Symbol, Terminal};
use crate::word::Word;

pub const MAX_SIZE: usize = 99;

pub trait WordGenerator {
    fn generate(&self) -> Result<Word, GenerationError>;
}

pub enum GenerationError {
    WordTooLong,
    InvalidTerminal(char),
    InvalidNonterminal(char),
}

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

impl<G: Grammar> RandomWordGenerator<G> {
    fn handle_terminal(
        &self,
        mut grammar: G,
        terminal: Terminal,
        mut content: String,
    ) -> Result<(G, String), GenerationError> {
        if content.len() >= MAX_SIZE {
            return Err(GenerationError::WordTooLong);
        }

        grammar = match grammar.handle(terminal) {
            Ok(grammar) => grammar,
            Err(InvalidTerminal(id)) => {
                return Err(GenerationError::InvalidTerminal(id));
            }
        };

        match grammar.semantic(terminal) {
            Ok(semantic) => println!("  → {}", semantic),
            Err(InvalidTerminal(id)) => {
                return Err(GenerationError::InvalidTerminal(id));
            }
        };

        content.push(terminal.id());
        return Ok((grammar, content));
    }

    fn handle_nonterminal(
        &self,
        grammar: G,
        nonterminal: Nonterminal,
        mut pending_symbols: Vec<Symbol>,
    ) -> Result<(G, Vec<Symbol>), GenerationError> {
        let productions = match grammar.productions(nonterminal) {
            Ok(productions) => productions,
            Err(InvalidNonterminal(id)) => {
                return Err(GenerationError::InvalidNonterminal(id));
            }
        };

        let selected_index = rand::random_range(0..productions.len());
        let selected_production = &productions[selected_index];

        for next_symbol in selected_production.symbols().iter().rev() {
            pending_symbols.push(*next_symbol);
        }

        return Ok((grammar, pending_symbols));
    }
}

impl<G: Grammar> WordGenerator for RandomWordGenerator<G> {
    fn generate(&self) -> Result<Word, GenerationError> {
        let mut content = String::new();
        let mut grammar = (self.create_grammar)();
        let mut pending_symbols = vec![Symbol::Nonterminal(INIT_SYMBOL)];

        while let Some(symbol) = pending_symbols.pop() {
            match symbol {
                Symbol::Terminal(terminal) => {
                    (grammar, content) = self.handle_terminal(grammar, terminal, content)?;
                }
                Symbol::Nonterminal(nonterminal) => {
                    (grammar, pending_symbols) =
                        self.handle_nonterminal(grammar, nonterminal, pending_symbols)?;
                }
            };
        }

        return Ok(Word::new(content));
    }
}
