use crate::grammar::{Grammar, GrammarError};
use crate::symbol::{INIT_SYMBOL, Symbol};
use crate::word::Word;

pub const MAX_SIZE: usize = 99;

pub trait WordGenerator {
    fn generate(&self) -> Result<Word, GenerationError>;
}

pub enum GenerationError {
    WordTooLong,
    InvalidNonterminal,
}

/**
 *  Generates a random word for a specified grammar.
 */
pub struct RandomWordGenerator<'grammar, G: Grammar> {
    grammar: &'grammar G,
}

impl<'grammar, G: Grammar> RandomWordGenerator<'grammar, G> {
    pub fn new(grammar: &'grammar G) -> Self {
        return RandomWordGenerator { grammar };
    }
}

impl<'grammar, G: Grammar> WordGenerator for RandomWordGenerator<'grammar, G> {
    fn generate(&self) -> Result<Word, GenerationError> {
        let mut content = String::new();
        let mut pending_symbols = vec![INIT_SYMBOL];

        while let Some(symbol) = pending_symbols.pop() {
            match symbol {
                Symbol::Terminal(character) => {
                    if content.len() >= MAX_SIZE {
                        return Err(GenerationError::WordTooLong);
                    }

                    content.push(character);

                    /* Print required by the teacher */
                    println!("  → Terminal {} added to word.", character);
                }

                nonterminal => {
                    let productions = match self.grammar.productions(nonterminal) {
                        Ok(productions) => productions,
                        Err(GrammarError::InvalidSymbol) => {
                            return Err(GenerationError::InvalidNonterminal);
                        }
                    };

                    let selected_index = rand::random_range(0..productions.len());
                    let selected_production = &productions[selected_index];

                    for next_symbol in selected_production.symbols().iter().rev() {
                        pending_symbols.push(*next_symbol);
                    }
                }
            };
        }

        return Ok(Word::new(content));
    }
}
