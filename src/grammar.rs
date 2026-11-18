use crate::production::Production;
use crate::symbol::Symbol;

pub trait Grammar {
    fn semantic(&self, symbol: Symbol) -> Result<String, GrammarError>;
    fn productions(&self, symbol: Symbol) -> Result<Vec<Production>, GrammarError>;
}

pub enum GrammarError {
    InvalidSymbol,
}
