pub const INIT_SYMBOL: Symbol = Symbol::NonTerminal('S');

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Symbol {
    Terminal(char),
    NonTerminal(char),
}
