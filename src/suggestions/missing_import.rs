use std::io::{Seek, SeekFrom, Read, Result, Write};
use super::{AppliedFixes, Suggestion};
use std::fs::OpenOptions;
use std::path::PathBuf;
use serde_json::Value;
use state::State;

#[derive(Default)]
pub struct MissingImport {
    title: String,
    file: String,
    suggestions: Vec<String>,
}

impl MissingImport {
    fn try_apply_option(&mut self, state: &mut State, option: String) -> Result<()> {
        let mut path = PathBuf::from(&state.working_directory);
        path.push(&self.file);

        let path = path.into_os_string();

        let option = option.trim().trim_matches('`');

        if state.applied_suggestions.iter().any(|i| {
            if let &AppliedFixes::AddedUsing { file: ref old_path, ref using } = i {
                if old_path == &path && using == option {
                    return true
                }
            }
            false
        }) {
            return Ok(());
        }

        state.applied_suggestions.push(
            AppliedFixes::AddedUsing { file: path.clone(), using: option.to_string() }
        );

        let mut file = OpenOptions::new().read(true)
            .write(true)
            .open(&path)?;
        let mut contents = String::new();
        file.read_to_string(&mut contents)?;
        contents = format!("{}\n{}", option, contents);
        file.seek(SeekFrom::Start(0))?;
        file.write_all(contents.as_bytes())?;

        Ok(())
    }
}

impl Suggestion for MissingImport {
    fn initialize(&mut self, json: &Value) -> bool {
        let message = match json.pointer("/message/children/0/message") {
            Some(&Value::String(ref str)) => str,
            _ => return false,
        };

        let mut lines = message.lines();
        let description = lines.next();
        if description !=
           Some("possible candidates are found in other modules, you can import them into scope:") &&
           description !=
           Some("possible candidate is found in another module, you can import it into scope:") {
            return false;
        }

        let options = lines.map(String::from).collect::<Vec<String>>();

        let filename = match json.pointer("/message/spans/0/file_name").and_then(Value::as_str) {
            Some(s) => s,
            None => return false,
        };
        let line_number = match json.pointer("/message/spans/0/line_start")
                  .and_then(Value::as_u64) {
            Some(s) => s,
            None => return false,
        };
        let code = match json.pointer("/message/spans/0/text/0/text").and_then(Value::as_str) {
            Some(s) => s,
            None => return false,
        };
        let text = match json.pointer("/message/message").and_then(Value::as_str) {
            Some(s) => s,
            None => return false,
        };

        self.file = filename.to_string();
        self.title = format!("{} line {}\n{}\n\n{}", filename, line_number, code, text);
        self.suggestions = options;

        true
    }

    fn title(&self) -> &str {
        &self.title
    }

    fn options(&self) -> Vec<String> {
        self.suggestions.clone()
    }


    fn apply_option(&mut self, state: &mut State, option: String) -> bool {
        if let Err(e) = self.try_apply_option(state, option) {
            println!("Could not apply: {}", e);
            false
        } else {
            true
        }
    }
}
