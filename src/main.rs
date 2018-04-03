mod nfa;
mod dfa;
extern crate regparser;
use std::mem;
use std::ffi::CStr;
use std::os::raw::{c_char, c_void};
use std::sync::Mutex;
use std::io::Write;
use std::str;
use regparser::parser::{Lexer, Parser};
use nfa::Nfa;
use dfa::Dfa;
#[macro_use]
extern crate lazy_static;

pub fn main() {}

extern "C" {
    fn print(n: String);
}

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
    let nfa = nfa::Nfa::re2nfa(&regex);
    let dfa = Dfa::nfa2dfa(&nfa);

    output.clear();
    write!(output, "{}\0", dfa.dot()).unwrap();
    output.as_ptr() as *mut c_char
}
