mod missing_import;

use std::ffi::OsString;
use serde_json::Value;
use state::State;

pub trait Suggestion {
    fn initialize(&mut self, json: &Value) -> bool;
    fn title(&self) -> &str;
    fn options(&self) -> Vec<String>;
    fn apply_option(&mut self, state: &mut State, option: String) -> bool;
}

pub fn get_all_suggestions() -> Vec<Box<Suggestion>> {
    vec![Box::new(self::missing_import::MissingImport::default())]
}

pub enum AppliedFixes {
    Unknown,
    AddedUsing { file: OsString, using: String },
}
