use crate::production::Production;
use crate::symbol::{INIT_SYMBOL, Symbol};

pub trait Grammar {
    fn productions(&self, symbol: Symbol) -> Result<Vec<Production>, GrammarError>;
}

pub enum GrammarError {
    InvalidSymbol,
}

/**
 * Devil May Cry Grammar:
 *  S → aS | rS | pS | lAS | lA | aaaa | e
 *  A → e | aB | lAA
 *  B → e | aC | lAA
 *  C → e | aD | lAA
 *  D → e | a  | lAA
 */
pub struct DMCGrammar;

impl DMCGrammar {
    pub fn new() -> Self {
        return DMCGrammar {};
    }

    fn ground_productions(&self) -> Vec<Production> {
        return vec![
            Production::new(vec![Symbol::Terminal('a'), INIT_SYMBOL]),
            Production::new(vec![Symbol::Terminal('r'), INIT_SYMBOL]),
            Production::new(vec![Symbol::Terminal('p'), INIT_SYMBOL]),
            Production::new(vec![
                Symbol::Terminal('l'),
                Symbol::Nonterminal('A'),
                INIT_SYMBOL,
            ]),
            Production::new(vec![Symbol::Terminal('l'), Symbol::Nonterminal('A')]),
            Production::new(vec![
                Symbol::Terminal('a'),
                Symbol::Terminal('a'),
                Symbol::Terminal('a'),
                Symbol::Terminal('a'),
            ]),
            Production::new(vec![Symbol::Terminal('e')]),
        ];
    }

    fn aerial_productions(&self, next_state: Option<Symbol>) -> Vec<Production> {
        let attack_symbols = match next_state {
            Some(state) => vec![Symbol::Terminal('a'), state],
            None => vec![Symbol::Terminal('a')],
        };

        vec![
            Production::new(vec![Symbol::Terminal('e')]),
            Production::new(attack_symbols),
            Production::new(vec![
                Symbol::Terminal('l'),
                Symbol::Nonterminal('A'),
                Symbol::Nonterminal('A'),
            ]),
        ]
    }
}

impl Grammar for DMCGrammar {
    fn productions(&self, symbol: Symbol) -> Result<Vec<Production>, GrammarError> {
        match symbol {
            INIT_SYMBOL => Ok(self.ground_productions()),
            Symbol::Nonterminal('A') => Ok(self.aerial_productions(Some(Symbol::Nonterminal('B')))),
            Symbol::Nonterminal('B') => Ok(self.aerial_productions(Some(Symbol::Nonterminal('C')))),
            Symbol::Nonterminal('C') => Ok(self.aerial_productions(Some(Symbol::Nonterminal('D')))),
            Symbol::Nonterminal('D') => Ok(self.aerial_productions(None)),
            _ => Err(GrammarError::InvalidSymbol),
        }
    }
}
