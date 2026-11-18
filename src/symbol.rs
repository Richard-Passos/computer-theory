pub const INIT_SYMBOL: Symbol = Symbol::Nonterminal('S');

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Symbol {
    Terminal(char),
    Nonterminal(char),
}
