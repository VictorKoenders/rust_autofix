use std::io::{Seek, SeekFrom, Read, Result, Write};
use std::fs::OpenOptions;
use super::Suggestion;
use serde_json::Value;

#[derive(Default)]
pub struct MissingImport {
    title: String,
    file: String,
    suggestions: Vec<String>,
}

impl MissingImport {
    fn try_apply_option(&mut self, option: String) -> Result<()> {
        let file = format!("d:/Development/Personal/rust_megabouncer/irc_connector/{}", self.file).replace("\\", "/");

        let ref option = option.trim().trim_matches('`');

        let mut file = OpenOptions::new().read(true).write(true).open(&file)?;
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
            _ => return false
        };

        let mut lines = message.lines();
        if lines.next() != Some("possible candidates are found in other modules, you can import them into scope:") { return false; }

        let options = lines.map(String::from).collect::<Vec<String>>();

        let filename = match json.pointer("/message/spans/0/file_name").and_then(Value::as_str){
            Some(s) => s,
            None => return false
        };
        let line_number = match json.pointer("/message/spans/0/line_start").and_then(Value::as_u64){
            Some(s) => s,
            None => return false
        };
        let code = match json.pointer("/message/spans/0/text/0/text").and_then(Value::as_str){
            Some(s) => s,
            None => return false
        };
        let text = match json.pointer("/message/message").and_then(Value::as_str){
            Some(s) => s,
            None => return false
        };

        self.file = filename.to_string();
        self.title = format!("{} line {}\n{}\n\n{}", filename, line_number, code, text);
        self.suggestions = options;

        true
    }

    fn title(&self) -> String {
        self.title.clone()
    }

    fn options(&self) -> Vec<String> {
        self.suggestions.clone()
    }


    fn apply_option(&mut self, option: String){
        if let Err(e) = self.try_apply_option(option){
            println!("Could not apply: {}", e);
        }
    }
}

