pub const INIT_SYMBOL: Nonterminal = Nonterminal('S');

#[derive(Clone, Copy)]
pub enum Symbol {
    Terminal(Terminal),
    Nonterminal(Nonterminal),
}

#[derive(Clone, Copy, PartialEq)]
pub struct Terminal(pub char);

#[derive(Clone, Copy, PartialEq)]
pub struct Nonterminal(pub char);

impl Terminal {
    pub const fn id(&self) -> char {
        return self.0;
    }
}

impl Nonterminal {
    pub const fn id(&self) -> char {
        return self.0;
    }
}
