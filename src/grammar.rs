use crate::production::Production;
use crate::symbol::{Nonterminal, Terminal};

pub trait Grammar {
    /**
     * Process the terminal and updates the grammar state.
     *
     * # Errors
     * Returns `InvalidTerminal` if the grammar does not recognize the terminal.
     */
    fn handle(&self, terminal: Terminal) -> Result<Self, InvalidTerminal>
    where
        Self: Sized;

    /**
     * Describes what the terminal means in the current grammar state.
     *
     * # Errors
     * Returns `InvalidTerminal` if the grammar does not recognize the terminal.
     */
    fn semantic(&self, terminal: Terminal) -> Result<String, InvalidTerminal>;

    /**
     * Returns the productions of a nonterminal.
     *
     * # Errors
     * Returns `InvalidNonterminal` if the grammar does not recognize the nonterminal.
     */
    fn productions(&self, nonterminal: Nonterminal) -> Result<Vec<Production>, InvalidNonterminal>;
}

pub struct InvalidTerminal(pub char);

pub struct InvalidNonterminal(pub char);
