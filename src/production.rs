use crate::symbol::Symbol;

#[derive(Clone)]
pub struct Production {
    symbols: Vec<Symbol>,
}

impl Production {
    pub fn new(symbols: Vec<Symbol>) -> Self {
        Self { symbols }
    }

    pub fn symbols(&self) -> &[Symbol] {
        &self.symbols
    }
}
