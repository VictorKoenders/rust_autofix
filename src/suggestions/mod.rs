mod missing_import;

use serde_json::Value;
use state::State;


pub trait Suggestion {
    fn initialize(&mut self, json: &Value) -> bool;
    fn title(&self) -> &str;
    fn options(&self) -> Vec<String>;
    fn apply_option(&mut self, state: &State, option: String);
}

pub fn get_all_suggestions() -> Vec<Box<Suggestion>> {
    vec![
        Box::new(self::missing_import::MissingImport::default())
    ]
}
