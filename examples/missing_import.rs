extern crate autofix;

use std::io::Read;

fn main() {
    let mut state = autofix::state::State::new();
    state.working_directory = String::from("examples/missing_import");
    state.mdoe = autofix::state::RunMode::Run;

    autofix::run_with_state(&mut state);

    //compare_file("examples/missing_import/src/main.rs",
    //             "examples/missing_import/src/main.correct.rs");

    std::fs::copy("examples/missing_import/src/main.incorrect.rs",
                  "examples/missing_import/src/main.rs")
            .unwrap();

    std::fs::remove_dir_all("examples/missing_import/target").unwrap();
}

#[allow(dead_code)]
fn compare_file(first_file: &str, second_file: &str) {
    println!("Comparing {} with {}", first_file, second_file);

    let mut first_contents = String::new();
    let mut second_contents = String::new();

    std::fs::File::open(first_file).unwrap().read_to_string(&mut first_contents).unwrap();
    std::fs::File::open(second_file).unwrap().read_to_string(&mut second_contents).unwrap();

    println!("Files match: {}",
             if first_contents == second_contents {
                 "yes"
             } else {
                 "no"
             });
}
