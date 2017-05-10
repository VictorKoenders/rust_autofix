extern crate serde_json;

use std::process::{Command, Output};
use std::io::{Read, Write};

mod suggestions;

fn main() {
    let mut suggestions = suggestions::get_all_suggestions();

    println!("Compiling in the background, this might take a second...");
    let output: Output = Command::new("cargo")
                              .arg("build")
                              .arg("--message-format=json")
                              .current_dir("d:/Development/Personal/rust_megabouncer/irc_connector")
                              .output()
                              .unwrap();
    let s = String::from_utf8_lossy(&output.stdout);
    let mut stdin = std::io::stdin();
    'mainloop: for (index, line) in s.lines().enumerate() {
        let json: serde_json::Value = match serde_json::from_str(line) {
            Ok(j) => j,
            Err(e) => {
                println!("Could not parse JSON at line {}: {:?}", index, e);
                println!("{}", line);
                println!("Full output can be found in compile_log.txt");
                std::fs::File::create("compile_log.txt").unwrap().write_all(s.as_bytes()).unwrap();
                break;
            }
        };

        for suggestion in &mut suggestions {
            if !suggestion.initialize(&json) { continue; }
            println!("{}", suggestion.title());
            let options = suggestion.options();

            loop {
                for (index, line) in options.iter().enumerate() {
                    println!("{}) {}", index, line);
                }
                println!();
                println!("s) Skip");
                println!("q) Quit");

                let mut byte = [0u8;1];
                stdin.read(&mut byte).unwrap();

                match byte[0] {
                    n if n >= b'0' && n <= b'9' && (n - b'0') as usize <= options.len() => {
                        let option = options[(n - b'0') as usize].clone();
                        suggestion.apply_option(option);
                        break;
                    },
                    b's' => {
                        break;
                    },
                    b'q' => {
                        break 'mainloop;
                    },
                    _ => {}
                }
            }
            break;
        }
    }
}
