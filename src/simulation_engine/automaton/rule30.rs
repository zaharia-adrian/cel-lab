use super::{Automaton, Condition, Rule};

pub fn create() -> Automaton {
    let neighbors = vec![
        (-1, 0),      (1, 0),  
    ];

    let rules = vec![
        // 111 -> 0
        Rule {
            crr_state: Some(1),
            next_state: 0,
            condition: Condition::Pattern(vec![1, 1]),
        },
        // 110 -> 0
        Rule {
            crr_state: Some(1),
            next_state: 0,
            condition: Condition::Pattern(vec![1, 0]),
        },
        // 101 -> 0
        Rule {
            crr_state: Some(0),
            next_state: 0,
            condition: Condition::Pattern(vec![1, 1]),
        },
        // 100 -> 1
        Rule {
            crr_state: Some(0),
            next_state: 1,
            condition: Condition::Pattern(vec![1, 0]),
        },
        // 011 -> 1
        Rule {
            crr_state: Some(1),
            next_state: 1,
            condition: Condition::Pattern(vec![0, 1]),
        },
        // 010 -> 1
        Rule {
            crr_state: Some(1),
            next_state: 1,
            condition: Condition::Pattern(vec![0, 0]),
        },
        // 001 -> 1
        Rule {
            crr_state: Some(0),
            next_state: 1,
            condition: Condition::Pattern(vec![0, 1]),
        },
        // 000 -> 0
        Rule {
            crr_state: Some(0),
            next_state: 0,
            condition: Condition::Pattern(vec![0, 0]),
        },
    ];

    Automaton { neighbors, rules}
}