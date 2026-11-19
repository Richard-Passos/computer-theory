use crate::word::Word;

pub const MAX_SIZE: usize = 99;

pub trait WordGenerator {
    fn generate(&self) -> Result<Word, GenerationError>;
}

pub enum GenerationError {
    WordTooLong(usize),
    InvalidTerminal(char),
    InvalidNonterminal(char),
}
