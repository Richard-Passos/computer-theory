mod state;
mod symbol;

use self::state::DMCCombatState;
use self::symbol::{DMCNonterminal, DMCTerminal};
use crate::grammar::{Grammar, InvalidNonterminal, InvalidTerminal};
use crate::production::Production;
use crate::symbol::{Nonterminal, Terminal};

use DMCNonterminal::{
    AirWithFourAttacks, AirWithOneAttack, AirWithThreeAttacks, AirWithTwoAttacks, Ground,
};
use DMCTerminal::{Attack, Launch, Roll, Taunt, Thrust};

/**
 * Here we are mapping the context-free grammar (CFG) to a struct.
 *  → Each terminal has a semantic text so it can be printed.
 *  → Each nonterminal has its productions so it can be used to generate a word.
 *
 * Devil May Cry Grammar:
 * ```text
 *  S → aS | rS | pS | lAS | lA | aaaa | e
 *  A → e | aB | lAA
 *  B → e | aC | lAA
 *  C → e | aD | lAA
 *  D → e | a  | lAA
 *
 *  a = Attack
 *  e = Thrust
 *  l = Launch
 *  r = Roll
 *  p = Taunt
 *
 *  S = Ground
 *  A = AirWithOneAttack
 *  B = AirWithTwoAttacks
 *  C = AirWithThreeAttacks
 *  D = AirWithFourAttacks
 * ```
 */
pub struct DMCGrammar {
    state: DMCCombatState,
}

impl Grammar for DMCGrammar {
    fn handle(&self, terminal: Terminal) -> Result<Self, InvalidTerminal> {
        let state = self.state.handle(terminal)?;
        return Ok(Self::from(state));
    }

    fn semantic(&self, terminal: Terminal) -> Result<String, InvalidTerminal> {
        let terminal = DMCTerminal::from(terminal)?;
        let mut semantic = String::from(format!(
            "{} | Level: {:02} | Attacks: {:02} | ",
            terminal.id(),
            self.state.level(),
            self.state.attacks()
        ));

        semantic.push_str(match terminal {
            Attack => "Strikes the enemy with a basic attack.",
            Thrust => "Delivers a finishing thrust, may descend one level.",
            Launch => "Launches himself and the enemy one level higher.",
            Roll => "Rolls to dodge an attack.",
            Taunt => "Taunts the enemy to build style.",
        });

        return Ok(semantic);
    }

    fn productions(&self, nonterminal: Nonterminal) -> Result<Vec<Production>, InvalidNonterminal> {
        let nonterminal = DMCNonterminal::from(nonterminal)?;

        return match nonterminal {
            Ground => Ok(self.ground_productions()),
            AirWithOneAttack => Ok(self.aerial_productions(Some(AirWithTwoAttacks))),
            AirWithTwoAttacks => Ok(self.aerial_productions(Some(AirWithThreeAttacks))),
            AirWithThreeAttacks => Ok(self.aerial_productions(Some(AirWithFourAttacks))),
            AirWithFourAttacks => Ok(self.aerial_productions(None)),
        };
    }
}

impl DMCGrammar {
    pub fn new() -> Self {
        return Self {
            state: DMCCombatState::new(),
        };
    }

    pub fn from(state: DMCCombatState) -> Self {
        return Self { state };
    }

    fn ground_productions(&self) -> Vec<Production> {
        return vec![
            Production::new(vec![Attack.symbol(), Ground.symbol()]),
            Production::new(vec![Roll.symbol(), Ground.symbol()]),
            Production::new(vec![Taunt.symbol(), Ground.symbol()]),
            Production::new(vec![
                Launch.symbol(),
                AirWithOneAttack.symbol(),
                Ground.symbol(),
            ]),
            Production::new(vec![Launch.symbol(), AirWithOneAttack.symbol()]),
            Production::new(vec![Attack.symbol(); 4]),
            Production::new(vec![Thrust.symbol()]),
        ];
    }

    fn aerial_productions(&self, next_state: Option<DMCNonterminal>) -> Vec<Production> {
        let attack_symbols = match next_state {
            Some(state) => vec![Attack.symbol(), state.symbol()],
            None => vec![Attack.symbol()],
        };

        vec![
            Production::new(vec![Thrust.symbol()]),
            Production::new(attack_symbols),
            Production::new(vec![
                Launch.symbol(),
                AirWithOneAttack.symbol(),
                AirWithOneAttack.symbol(),
            ]),
        ]
    }
}
