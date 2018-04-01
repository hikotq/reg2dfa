use std::collections::{HashMap, HashSet};
use nfa::{Nfa, StateSet};

#[derive(Debug)]
pub struct Dfa {
    pub transitions: HashMap<StateSet, HashMap<char, StateSet>>, 
    init_state: StateSet, 
    accept_state: StateSet, 
    states_num: usize, 
}

impl Dfa {
    pub fn nfa2dfa(nfa: &Nfa) -> Self {
        Dfa::construct(nfa)
    }

    fn construct(nfa: &Nfa) -> Self {
        let mut  dfa = Dfa { transitions: HashMap::new(), init_state: StateSet(HashSet::new()), accept_state: StateSet(HashSet::new()), states_num: 0 };
        let transitions: HashMap<HashSet<usize>, char>;
        let mut states_list: Vec<HashSet<usize>> = Vec::new();
        let mut nfa_init_state_set = HashSet::new();
        nfa_init_state_set.insert(0);
        dfa.init_state = StateSet(nfa.epsilon_expand(StateSet(nfa_init_state_set)));
        let mut done = HashSet::new();
        let dfa_init_state = dfa.init_state.clone();
        dfa.construct_recursive(nfa, dfa_init_state, &mut done);
        dfa
    }

    fn construct_recursive(&mut self, nfa: &Nfa, state_set: StateSet, done: &mut HashSet<StateSet>) {
        let t = nfa.subset_transitions(state_set.clone());
        {
        let t_val = t.values().clone();
        for next in t_val {
            if !done.contains(next) {
                done.insert(next.clone());
                self.construct_recursive(nfa, next.clone(), done);
            }
        }
        }
        self.transitions.insert(state_set, t);
    }
}
