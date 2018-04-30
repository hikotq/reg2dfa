#[macro_use]
extern crate lazy_static;
extern crate regex;
extern crate regparser;

use std::os::raw::c_char;
use std::sync::Mutex;
use std::io::Write;
use regex::{dfa::Dfa, nfa::Nfa};

pub fn main() {}

lazy_static! {
    static ref HEAP: Mutex<Vec<u8>> = Mutex::new(Vec::new());
}

#[no_mangle]
pub fn alloc(size: usize) -> *mut c_char {
    let mut output = HEAP.lock().unwrap();
    output.resize(size, 0);
    output.as_mut_ptr() as *mut c_char
}

#[no_mangle]
pub fn offset() -> *const c_char {
    let mut output = HEAP.lock().unwrap();
    output.as_mut_ptr() as *const c_char
}

#[no_mangle]
pub fn get_heap_len() -> usize {
    HEAP.lock().unwrap().len()
}

#[no_mangle]
pub fn get_dot() -> *mut c_char {
    let mut output = HEAP.lock().unwrap();
    let regex = output
        .clone()
        .iter()
        .map(|&b| b as char)
        .collect::<String>();
    let nfa = Nfa::re2nfa(&regex);
    let dfa = Dfa::nfa2dfa(&nfa);

    output.clear();
    write!(output, "{}\0", dfa.dot()).unwrap();
    output.as_ptr() as *mut c_char
}
