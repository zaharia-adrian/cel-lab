use super::{Automaton, Condition, Rule};

pub fn create() -> Automaton {
    let neighbors = vec![
        (-1, -1), (0, -1), (1, -1),
        (-1,  0),           (1,  0),
        (-1,  1), (0,  1), (1,  1),
    ];

    let rules = vec![
        // FIRING -> DYING
        Rule {
            crr_state: Some(1),
            next_state: 2,
            condition: Condition::Fallback,
        },

        // DYING -> OFF
        Rule {
            crr_state: Some(2),
            next_state: 0,
            condition: Condition::Fallback,
        },

        // OFF -> FIRING if exactly 2 neighbors are FIRING
        Rule {
            crr_state: Some(0),
            next_state: 1,
            condition: Condition::Counts {
                target_state: 1,
                target_counts: vec![2],
            },
        },

        // OFF remains OFF otherwise
        Rule {
            crr_state: Some(0),
            next_state: 0,
            condition: Condition::Fallback,
        },
    ];

    Automaton { neighbors, rules }
}
