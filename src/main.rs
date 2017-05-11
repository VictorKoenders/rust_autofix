extern crate rust_autofix;

use rust_autofix::state::State;

fn main() {
    let state = State::new();
    rust_autofix::run_with_state(&state);
}
