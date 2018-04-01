mod nfa;
mod dfa;
extern crate regparser;
use nfa::Nfa;
use dfa::Dfa;
fn main() {
    let regex = "(a|b)*";
    let nfa = nfa::Nfa::re2nfa(regex);
    nfa.print();
    nfa.write();
    //let dfa = Dfa::nfa2dfa(&nfa);
    //println!("{:?}", dfa.transitions);
}
