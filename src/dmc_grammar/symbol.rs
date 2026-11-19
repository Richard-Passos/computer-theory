use crate::grammar::{InvalidNonterminal, InvalidTerminal};
use crate::symbol::{INIT_SYMBOL, Nonterminal, Symbol, Terminal};

#[derive(Clone, Copy, PartialEq)]
#[repr(u8)]
pub enum DMCTerminal {
    Attack = b'a',
    Thrust = b'e',
    Launch = b'l',
    Roll = b'r',
    Taunt = b't',
}

#[derive(Clone, Copy, PartialEq)]
#[repr(u8)]
pub enum DMCNonterminal {
    Ground = INIT_SYMBOL.id() as u8,
    AirWithOneAttack = b'A',
    AirWithTwoAttacks = b'B',
    AirWithThreeAttacks = b'C',
    AirWithFourAttacks = b'D',
}

impl DMCTerminal {
    pub fn id(self) -> char {
        char::from(self as u8)
    }

    pub fn symbol(self) -> Symbol {
        return Symbol::Terminal(Terminal(self.id()));
    }

    pub fn from(terminal: Terminal) -> Result<Self, InvalidTerminal> {
        return match terminal {
            Terminal('a') => Ok(Self::Attack),
            Terminal('e') => Ok(Self::Thrust),
            Terminal('l') => Ok(Self::Launch),
            Terminal('r') => Ok(Self::Roll),
            Terminal('t') => Ok(Self::Taunt),
            _ => Err(InvalidTerminal(terminal.id())),
        };
    }
}

impl DMCNonterminal {
    pub fn id(self) -> char {
        char::from(self as u8)
    }

    pub fn symbol(self) -> Symbol {
        return Symbol::Nonterminal(Nonterminal(self.id()));
    }

    pub fn from(nonterminal: Nonterminal) -> Result<Self, InvalidNonterminal> {
        match nonterminal {
            INIT_SYMBOL => Ok(Self::Ground),
            Nonterminal('A') => Ok(Self::AirWithOneAttack),
            Nonterminal('B') => Ok(Self::AirWithTwoAttacks),
            Nonterminal('C') => Ok(Self::AirWithThreeAttacks),
            Nonterminal('D') => Ok(Self::AirWithFourAttacks),
            _ => Err(InvalidNonterminal(nonterminal.id())),
        }
    }
}
