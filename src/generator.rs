use crate::grammar::{Grammar, GrammarError};
use crate::symbol::{INIT_SYMBOL, Symbol};
use crate::word::Word;

pub const MAX_SIZE: usize = 99;

pub trait WordGenerator {
    fn generate(&self) -> Result<Word, GenerationError>;
}

pub enum GenerationError {
    WordTooLong,
    InvalidTerminal,
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
                Symbol::Terminal(id) => {
                    let terminal = Symbol::Terminal(id);

                    if content.len() >= MAX_SIZE {
                        return Err(GenerationError::WordTooLong);
                    }

                    match self.grammar.semantic(terminal) {
                        Ok(semantic) => println!("  → {}: {}", id, semantic),
                        Err(GrammarError::InvalidSymbol) => {
                            return Err(GenerationError::InvalidTerminal);
                        }
                    };

                    content.push(id);
                }

                Symbol::Nonterminal(id) => {
                    let nonterminal = Symbol::Nonterminal(id);
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
