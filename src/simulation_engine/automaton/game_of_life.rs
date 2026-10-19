
use super::{Automaton, Condition, Rule};

pub fn create() -> Automaton {
    let neighbors = vec![
        (-1, -1), (0, -1), (1, -1),
        (-1,  0),           (1,  0),
        (-1,  1), (0,  1), (1,  1),
    ];

    let rules = vec![
        Rule {
            crr_state: Some(1),
            next_state: 1,
            condition: Condition::Counts {
                target_state: 1,
                target_counts: vec![2, 3],
            },
        },
        Rule {
            crr_state: Some(0),
            next_state: 1,
            condition: Condition::Counts {
                target_state: 1,
                target_counts: vec![3],
            },
        },
        Rule {
            crr_state: None,
            next_state: 0,
            condition: Condition::Fallback,
        },
    ];

    Automaton { neighbors, rules}
}