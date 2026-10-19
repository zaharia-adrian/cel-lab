use crate::automaton::{Automaton, ParamValue, Rule};

pub fn create() -> Automaton {
    let mut params = HashMap::new();

    params.insert("num_states".into(), ParamValue::Integer(4));
    params.insert("threshold".into(), ParamValue::Integer(2));

    Automaton {
        neighbors: vec![
            (-1, -1), (0, -1), (1, -1),
            (-1,  0),          (1,  0),
            (-1,  1), (0,  1), (1,  1),
        ],
        rules: vec![
            Rule {
                crr_state: None,
                next_state: 0, // Calculated dynamically in step()
                condition: Condition::Cyclic,
            },
        ],
        params,
    }
}