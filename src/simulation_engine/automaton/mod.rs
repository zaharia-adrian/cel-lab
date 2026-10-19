pub mod brians_brain;
pub mod game_of_life;
pub mod rule30;

use std::collections::HashMap;

#[derive(Debug, Clone)]
pub enum Condition {
    Counts {
        target_state: u8,
        target_counts: Vec<u8>,
    },
    Pattern(Vec<u8>),
    Fallback,
}



#[derive(Debug, Clone)]
pub struct Rule {
    pub crr_state: Option<u8>,
    pub next_state: u8,
    pub condition: Condition,
}

pub struct Automaton {
    pub neighbors: Vec<(isize, isize)>,
    pub rules: Vec<Rule>,
}

