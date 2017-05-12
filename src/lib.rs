extern crate serde_json;

use std::process::{Command, Output};
use std::io::{Write, Read};
use std::path::PathBuf;
use serde_json::Value;
use state::State;

mod suggestions;
pub mod state;


pub fn run_with_state(state: &mut State) {
    let mut should_build = true;
    let mut suggestions = suggestions::get_all_suggestions();

    let mut dir = PathBuf::new();
    dir.push(std::env::current_dir().unwrap());
    dir.push(state.working_directory.clone());

    'mainloop: while should_build {

        println!("Compiling in the background, this might take a second...");

        let output: Output = match Command::new("cargo")
                  .arg(state.mode.to_string())
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

        println!("Cargo exited with code {:?}", output.status.code());

        let s = String::from_utf8_lossy(&output.stdout);

        if state.output_compile_log {
            std::fs::File::create("compile.log").unwrap().write_all(s.as_bytes()).unwrap();
        }

        let mut print_output = false;

        for line in s.lines() {
            if print_output {
                println!("{}", line);
                continue;
            }
            let json: Value = match serde_json::from_str(line) {
                Ok(j) => j,
                Err(_) => {
                    println!("{}", line);
                    print_output = true;
                    continue;
                }
            };

            let mut was_handled = false;

            for mut suggestion in &mut suggestions {
                if suggestion.initialize(&json) {
                    was_handled = true;
                    match handle_suggestion(&mut suggestion, state) {
                        HandleSuggestionResult::Skipped => {
                            if should_build {
                                println!("cancelling subsequential builds");
                                should_build = false;
                            }
                        }
                        HandleSuggestionResult::Quit => {
                            break 'mainloop;
                        }
                        HandleSuggestionResult::Ok => {}
                    };
                    break;
                }
            }

            if !was_handled && print_error(&json) {
                should_build = false;
            }
        }

        if output.status.success() {
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
                     state: &mut State)
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
                if suggestion.apply_option(state, option) {
                    println!("Applied change succesfully");
                    return HandleSuggestionResult::Ok;
                } else {
                    println!("Could not apply change, quitting");
                    return HandleSuggestionResult::Quit;
                }
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
    let level = match json.pointer("/message/level").and_then(Value::as_str) {
        Some(l) => l,
        None => {
            // println!("Could not find message level: {}", json);
            return false;
        }
    };
    if level == "note" {
        return false;
    }
    if level == "error" {
        if let Some(message) = json.pointer("/message/message").and_then(Value::as_str) {
            if message == "aborting due to previous error" {
                return false;
            } else {
                if let Some(file_name) = json.pointer("/message/spans/0/file_name").and_then(Value::as_str) {
                    if let Some(line) = json.pointer("/message/spans/0/line_start").and_then(Value::as_u64) {
                        println!("{} line {}", file_name, line);
                    } else {
                        println!("{}", file_name);
                    }
                }
                println!("{}", message);
                return true;
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
