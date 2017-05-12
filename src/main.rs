extern crate autofix;

use autofix::state::State;

fn main() {
    let mut state = match State::from_args(&mut std::env::args()) {
        Some(s) => s,
        None => return,
    };
    autofix::run_with_state(&mut state);
}
