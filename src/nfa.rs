use regparser::parser::{Parser, Lexer, NodeType, Node};
use std::collections::{HashMap, HashSet};
use std::hash::{Hash, Hasher};


#[derive(PartialEq, Eq, Hash)]
pub enum Label {
    Epsilon, 
    Input(char), 
}

#[derive(Debug)]
struct State {
    transitions: HashMap<char, HashSet<usize>>, 
    epsilon_trnasitions: HashSet<usize>, 
    id: usize, 
    accept: bool,
}

#[derive(Debug)]
pub struct Nfa {
    states: Vec<State>, 
}

impl Nfa {
    pub fn re2nfa(regex: &str) -> Nfa {
        let lexer = Lexer::new(regex.trim());
        let parser = Parser::new(lexer);
        let mut syntax_tree = parser.struct_syntax_tree();
        if let Some(root) = syntax_tree.root {
            let mut nfa = Nfa { states: Vec::new() };
            nfa.add_state();
            nfa.construct(&root);
            nfa.add_state();
            let states_num = nfa.states.len();
            nfa.states[states_num - 2].epsilon_trnasitions.insert(states_num - 1);
            nfa.states[states_num - 1].accept = true;
            nfa
        } else {
            panic!();
        }
    }

    fn add_state(&mut self) {
        let state_num = self.states.len();
        self.states.push(
            State {
                transitions: HashMap::new(), 
                epsilon_trnasitions: HashSet::new(), 
                id: state_num, 
                accept: false, 
            });
    }

    fn construct(&mut self, node: &Node) {
        use self::NodeType::*;
        match node.node_type {
            OpUnion => {
                let states_num = self.states.len();
                self.states[states_num - 1].epsilon_trnasitions.insert(states_num);
                self.add_state();
                let states_num = self.states.len();
                self.states[states_num - 1].epsilon_trnasitions.insert(states_num+1);
                self.states[states_num - 1].epsilon_trnasitions.insert(states_num+2);
                let &Node {ref lhs, ref rhs, .. } = node;
                self.construct(lhs.as_ref().unwrap());
                self.construct(rhs.as_ref().unwrap());
                self.add_state();
                let states_num = self.states.len();
                self.states[states_num - 2].epsilon_trnasitions.insert(states_num - 1);
                self.states[states_num - 3].epsilon_trnasitions.insert(states_num - 1);
            }
            OpConcat => {
                let states_num = self.states.len();
                self.states[states_num - 1].epsilon_trnasitions.insert(states_num);
                self.add_state();
                let &Node {ref lhs, ref rhs, .. } = node;
                self.construct(lhs.as_ref().unwrap());
                let states_num = self.states.len();
                self.states[states_num - 1].epsilon_trnasitions.insert(states_num);
                self.construct(rhs.as_ref().unwrap());
                let states_num = self.states.len();
                self.states[states_num - 1].epsilon_trnasitions.insert(states_num);
                self.add_state()
            }
            OpStar => {
                let states_num = self.states.len();
                self.states[states_num - 1].epsilon_trnasitions.insert(states_num);
                self.add_state();
                let state_id = self.states.len() - 1;
                let &Node {ref lhs, ..} = node;
                self.construct(lhs.as_ref().unwrap());
                let states_num = self.states.len();
                self.states[states_num - 1].epsilon_trnasitions.insert(state_id);
            }
            Literal => {
                let states_num = self.states.len();
                self.states[states_num - 1].epsilon_trnasitions.insert(states_num);
                self.add_state();
                let states_num = self.states.len();
                let mut t = HashSet::new();
                t.insert(states_num);
                let &Node {ref value, ..} = node;
                self.states[states_num - 1].transitions.insert(value.as_ref().unwrap().chars().next().unwrap(), t);
                self.add_state();
                let states_num = self.states.len();
                self.states[states_num - 1].epsilon_trnasitions.insert(states_num);
            }
            _ => {panic!();}
        }
    }

    pub fn reachable_subsets(&self, state_id: usize) -> HashSet<usize> {
        let mut reachable_subsets = HashSet::new();
        for byte in (0 as u8)..=255 {
            let c = byte as char;
            if let Some(state_set) = self.states[state_id].transitions.get(&c) {
                reachable_subsets = reachable_subsets.union(state_set).cloned().collect();
            }
        }

        reachable_subsets = reachable_subsets.union(&self.states[state_id].epsilon_trnasitions).cloned().collect();
        reachable_subsets
    }

    pub fn epsilon_expand(&self, state_set: StateSet) -> HashSet<usize>{
        let mut queue = state_set.0.iter().cloned().collect::<Vec<usize>>();
        let mut done: HashSet<usize> = HashSet::new();
        while queue.len() != 0 {
            let state_id = queue.pop().unwrap();
            let next = self.states[state_id].epsilon_trnasitions.clone();
            done.insert(state_id);
            for next_state_id in next.iter() {
                if !done.contains(next_state_id) {
                    queue.push(*next_state_id);
                }
            }
        }
        done
    }

    pub fn subset_transitions(&self, reachable_states: StateSet) -> HashMap<char, StateSet> {
        let mut  transitions = HashMap::new();
        for byte in (0 as u8)..=255 {
            let c = byte as char;
            let mut t = HashSet::new();
            for id in reachable_states.0.iter() {
                if let Some(state_set) = self.states[*id].transitions.get(&c) {
                    t = t.union(state_set).cloned().collect();
                }
            }
            let t: HashSet<usize> = t.union(&self.epsilon_expand(StateSet(t.clone()))).cloned().collect();
            if !t.is_empty() {
                transitions.insert(c, StateSet(t));
            }
        }
        transitions
    }

    pub fn print(&self) {
        for state in self.states.iter() {
            println!("{:?}", state);
        }
    }

    pub fn write(&self) {
        let mut dot = r###"
digraph G {
rankdir=LR;
empty [label = "" shape = plaintext];
node [shape = doublecircle]; e1 e2;
node [shape = circle];
empty -> s0 [label = "開始"];
        "###.to_owned();
        let t_dot = "s{} -> s{} [label = \"{}\"]".to_owned();
        
        for (id, state) in self.states.iter().enumerate() {
            for (label, t_state_set) in state.transitions.iter() {
                for t_state in t_state_set.iter() {
                    dot.push_str(&format!("s{} -> s{} [label = \"{}\"]\n", id, t_state, label));
                }
            }
            for et_state in state.epsilon_trnasitions.iter() {
                dot.push_str(&format!("s{} -> s{} [label = \"{}\"]\n", id, et_state, "ε"));
            }
        }
        dot.push_str("}");
        println!("{}", dot);
    }
        
}

#[derive(Debug, Clone)]
pub struct StateSet(pub HashSet<usize>);

impl PartialEq for StateSet
{
    fn eq(&self, other: &StateSet) -> bool {
        self.0 == other.0
    }
}

impl Eq for StateSet
{
}

impl Hash for StateSet {
    fn hash<H>(&self, _state: &mut H)
    where
        H: Hasher,
    {
        for s in self.0.iter() {
            s.hash(_state);
        }
    }
}

