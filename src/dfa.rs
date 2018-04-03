use std::collections::{HashMap, HashSet};
use std::fs;
use std::io::{BufWriter, Write};
use nfa::{Nfa, StateSet};

#[derive(Debug)]
pub struct Dfa {
    pub transitions: HashMap<StateSet, HashMap<char, StateSet>>,
    init_state: StateSet,
    accept_state_set: HashSet<usize>,
    states_num: usize,
}

impl Dfa {
    pub fn nfa2dfa(nfa: &Nfa) -> Self {
        Dfa::construct(nfa)
    }

    fn construct(nfa: &Nfa) -> Self {
        let mut dfa = Dfa {
            transitions: HashMap::new(),
            init_state: StateSet(HashSet::new()),
            accept_state_set: HashSet::new(),
            states_num: 0,
        };
        for accept_state in nfa.states.iter().filter(|&state| state.accept) {
            dfa.accept_state_set.insert(accept_state.id);
        }
        let transitions: HashMap<HashSet<usize>, char>;
        let mut states_list: Vec<HashSet<usize>> = Vec::new();
        let mut nfa_init_state_set = HashSet::new();
        nfa_init_state_set.insert(0);
        dfa.init_state = StateSet(nfa.epsilon_expand(StateSet(nfa_init_state_set)));
        println!("{:?}", dfa.init_state);
        let mut done = HashSet::new();
        let dfa_init_state = dfa.init_state.clone();
        dfa.construct_recursive(nfa, dfa_init_state, &mut done);
        dfa
    }

    fn construct_recursive(
        &mut self,
        nfa: &Nfa,
        state_set: StateSet,
        done: &mut HashSet<StateSet>,
    ) {
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

    pub fn dot(&self) -> String {
        let mut dot = r###"
digraph G {
rankdir=LR;
empty [label = "" shape = plaintext];
"###
            .to_owned();
        let t_dot = "s{} -> s{} [label = \"{}\"]".to_owned();

        //各状態へのラベル付け
        let mut state_index_map: HashMap<StateSet, usize> = HashMap::new();
        let mut queue: Vec<StateSet> = Vec::new();
        queue.push(
            self.transitions
                .keys()
                .filter(|state_set| state_set.0.contains(&0))
                .nth(0)
                .unwrap()
                .clone(),
        );
        let mut done: HashSet<StateSet> = HashSet::new();
        while queue.len() != 0 {
            let state_set = queue.pop().unwrap();
            let n = state_index_map.keys().len();
            state_index_map.entry(state_set.clone()).or_insert(n);
            if let Some(t) = self.transitions.get(&state_set) {
                for t_state in t.values() {
                    if !done.contains(t_state) {
                        queue.push(t_state.clone());
                        done.insert(t_state.clone());
                    }
                }
            }
        }

        let mut accept_state_dot = "\nnode [shape = doublecircle]".to_owned();
        for state_set in self.transitions.keys() {
            if state_set.0.intersection(&self.accept_state_set).count() != 0 {
                //println!("{:?}", state_set.0.intersection(&self.accept_state_set));
                accept_state_dot.push_str(
                    &("s".to_owned() + &state_index_map.get(state_set).unwrap().to_string() + " "),
                );
            }
        }
        accept_state_dot.push_str(";\n");
        dot.push_str(&accept_state_dot);
        dot.push_str("node [shape = circle];\nempty -> s0 [label = \"start\"]\n");

        for i in 0..self.transitions.keys().len() * 2 {
            let mut t_keys = self.transitions
                .keys()
                .filter(|state_set| state_set.0.contains(&i));
            for state_set in t_keys {
                let n = state_index_map.keys().len();
                state_index_map.entry(state_set.clone()).or_insert(n);
            }
        }
        for (state_set, transitions) in self.transitions.iter() {
            for (label, t_state_set) in transitions.iter() {
                dot.push_str(&format!(
                    "s{} -> s{} [label = \"{}\"]\n",
                    state_index_map.get(&state_set).unwrap(),
                    state_index_map.get(&t_state_set).unwrap(),
                    label
                ));
            }
        }
        dot.push_str("}");
        dot
    }

    pub fn write(&self, file_name: &str) {
        let dot = self.dot();
        let mut f = BufWriter::new(fs::File::create(file_name).unwrap());
        f.write(dot.as_bytes()).unwrap();
    }
}
