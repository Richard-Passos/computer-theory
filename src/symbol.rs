pub const INIT_SYMBOL: Nonterminal = Nonterminal('S');

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Symbol {
    Terminal(Terminal),
    Nonterminal(Nonterminal),
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Terminal(pub char);

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Nonterminal(pub char);

impl Terminal {
    pub fn id(&self) -> char {
        return self.0;
    }
}
