use super::symbol::DMCTerminal;
use crate::grammar::InvalidTerminal;
use crate::symbol::Terminal;

pub struct DMCCombatState {
    level: usize,
    attacks: usize,
}

impl DMCCombatState {
    pub fn new() -> Self {
        return Self {
            level: 0,
            attacks: 0,
        };
    }

    pub fn from(other: &Self) -> Self {
        return Self {
            level: other.level(),
            attacks: other.attacks(),
        };
    }

    pub fn handle(&self, terminal: Terminal) -> Result<Self, InvalidTerminal> {
        let terminal = DMCTerminal::from(terminal)?;
        let mut state = Self::from(self);

        if state.should_reset_attacks(terminal) {
            state.reset_attacks();
        }

        match terminal {
            DMCTerminal::Attack => state.attack(),
            DMCTerminal::Launch => state.ascend(),
            DMCTerminal::Thrust => state.descend(),
            _ => (),
        }

        return Ok(state);
    }

    pub fn level(&self) -> usize {
        return self.level;
    }

    pub fn attacks(&self) -> usize {
        return self.attacks;
    }

    fn ascend(&mut self) {
        self.level += 1;
    }

    fn descend(&mut self) {
        if self.level > 0 {
            self.level -= 1;
        }
    }

    fn attack(&mut self) {
        self.attacks += 1;
    }

    fn reset_attacks(&mut self) {
        self.attacks = 0;
    }

    fn should_reset_attacks(&self, terminal: DMCTerminal) -> bool {
        return terminal != DMCTerminal::Attack;
    }
}
