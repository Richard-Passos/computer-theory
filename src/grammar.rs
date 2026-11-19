use crate::production::Production;
use crate::symbol::{Nonterminal, Terminal};

pub trait Grammar {
    fn handle(&self, terminal: Terminal) -> Result<Self, InvalidTerminal>
    where
        Self: Sized;
    fn semantic(&self, terminal: Terminal) -> Result<String, InvalidTerminal>;
    fn productions(&self, nonterminal: Nonterminal) -> Result<Vec<Production>, InvalidNonterminal>;
}

#[derive(Debug)]
pub struct InvalidTerminal;

#[derive(Debug)]
pub struct InvalidNonterminal;
