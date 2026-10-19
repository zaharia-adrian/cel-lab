pub mod automaton;
use automaton::{Automaton, Condition};


pub struct SimulationEngine {
    pub width: usize,
    pub height: usize,
    pub state: Vec<u8>,
    pub automaton: Automaton,
}

impl SimulationEngine {
    pub fn new(width: usize, height: usize, state: Vec<u8>, automaton: Automaton) -> Self {
        assert!(width > 0 && height > 0);
        assert_eq!(state.len(), width * height);

        Self {
            width,
            height,
            state,
            automaton,
        }
    }

    #[inline]
    fn index(&self, x: usize, y: usize) -> usize {
        y * self.width + x
    }

    pub fn get(&self, x: usize, y: usize) -> Option<u8> {
        if x >= self.width || y >= self.height {
            return None;
        }

        Some(self.state[self.index(x, y)])
    }

    pub fn set(&mut self, x: usize, y: usize, value: u8) {
        assert!(x < self.width && y < self.height);

        let index = self.index(x, y);
        self.state[index] = value;
    }

    fn get_neighbor_states(&self, x: usize, y: usize) -> Vec<u8> {
        self.automaton
            .neighbors
            .iter()
            .filter_map(|&(dx, dy)| {
                let nx = x as isize + dx;
                let ny = y as isize + dy;

                if nx < 0 || ny < 0 || nx >= self.width as isize || ny >= self.height as isize {
                    return None;
                }

                Some(self.state[self.index(nx as usize, ny as usize)])
            })
            .collect()
    }

    pub fn step(&mut self) {
        let mut next_state = self.state.clone();

        for y in 0..self.height {
            for x in 0..self.width {
                let index = self.index(x, y);
                let current = self.state[index];
                let neighbor_states = self.get_neighbor_states(x, y);

                for rule in &self.automaton.rules {
                    if let Some(required) = rule.crr_state {
                        if required != current {
                            continue;
                        }
                    }

                    let matches = match &rule.condition {
                        Condition::Counts {
                            target_state,
                            target_counts,
                        } => {
                            let count = neighbor_states
                                .iter()
                                .filter(|&&s| s == *target_state)
                                .count();

                            target_counts.iter().any(|&n| usize::from(n) == count)
                        }

                        Condition::Pattern(pattern) => pattern == &neighbor_states,

                        Condition::Fallback => true,
                    };

                    if matches {
                        next_state[index] = rule.next_state;
                        break;
                    }
                }
            }
        }

        self.state = next_state;
    }
}
