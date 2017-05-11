extern crate serde_json;

use std::process::{Command, Output};
use std::io::{Read, Write};
use std::path::PathBuf;
use serde_json::Value;
use state::State;

mod suggestions;
pub mod state;


pub fn run_with_state(state: &State) {
    let mut skipped_or_quit = false;
    let mut suggestions = suggestions::get_all_suggestions();

    let mut dir = PathBuf::new();
    dir.push(std::env::current_dir().unwrap());
    dir.push(state.working_directory.clone());

    while !skipped_or_quit {

        println!("Compiling in the background, this might take a second...");


        let output: Output = match Command::new("cargo")
                  .arg("build")
                  .arg("--message-format=json")
                  .current_dir(dir.clone())
                  .output() {
            Ok(o) => o,
            Err(e) => {
                println!("Could not execute cargo: {}", e);
                println!("Working directory: {:?}", dir);
                return;
            }
        };

        let s = String::from_utf8_lossy(&output.stdout);

        let mut has_had_errors = false;

        'mainloop: for (index, line) in s.lines().enumerate() {
            let json: Value = match serde_json::from_str(line) {
                Ok(j) => j,
                Err(e) => {
                    println!("Could not parse JSON at line {}: {:?}", index, e);
                    println!("{}", line);
                    println!("Full output can be found in compile_log.txt");
                    std::fs::File::create("compile_log.txt")
                        .unwrap()
                        .write_all(s.as_bytes())
                        .unwrap();
                    break;
                }
            };

            let mut was_handled = false;

            for mut suggestion in &mut suggestions {
                if suggestion.initialize(&json) {
                    has_had_errors = true;
                    was_handled = true;
                    match handle_suggestion(&mut suggestion, state) {
                        HandleSuggestionResult::Skipped => skipped_or_quit = true,
                        HandleSuggestionResult::Quit => {
                            skipped_or_quit = true;
                            break 'mainloop;
                        }
                        HandleSuggestionResult::Ok => {}
                    };
                    break;
                }
            }

            if !was_handled && print_error(&json) {
                has_had_errors = true;
            }
        }

        if !has_had_errors {
            break;
        }
    }
}

enum HandleSuggestionResult {
    Ok,
    Skipped,
    Quit,
}

fn handle_suggestion(suggestion: &mut Box<suggestions::Suggestion>,
                     state: &State)
                     -> HandleSuggestionResult {
    println!("{}", suggestion.title());
    let options = suggestion.options();

    let mut stdin = std::io::stdin();

    loop {
        for (index, line) in options.iter().enumerate() {
            println!("{}) {}", index, line);
        }
        println!();
        println!("s) Skip");
        println!("q) Quit");

        let mut byte = [0u8; 1];
        stdin.read_exact(&mut byte).unwrap();

        match byte[0] {
            n if n >= b'0' && n <= b'9' && (n - b'0') as usize <= options.len() => {
                let option = options[(n - b'0') as usize].clone();
                suggestion.apply_option(state, option);
                return HandleSuggestionResult::Ok;
            }
            b's' => {
                return HandleSuggestionResult::Skipped;
            }
            b'q' => {
                return HandleSuggestionResult::Quit;
            }
            _ => {}
        }
    }
}

fn print_error(json: &Value) -> bool {
    let level = match json.pointer("/message/children/0/level").and_then(Value::as_str) {
        Some(l) => l,
        None => {
            //println!("Could not find message level: {}", json);
            return false;
        }
    };
    if level == "note" {
        return false;
    }
    if level == "error" {
        if let Some(message) = json.pointer("/message/children/0/message").and_then(Value::as_str) {
            if message == "aborting due to previous error" {
                return false;
            }
        }
    }
    println!();
    println!("NOT IMPLEMENTED: ");
    println!("{}", json);
    println!("Please report this at http://www.github.com/victorkoenders/rust_autofix");
    println!();
    true
}
