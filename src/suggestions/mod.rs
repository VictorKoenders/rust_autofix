use serde_json::Value;

mod missing_import;

pub trait Suggestion {
    fn initialize(&mut self, json: &Value) -> bool;
    fn title(&self) -> String;
    fn options(&self) -> Vec<String>;
    fn apply_option(&mut self, option: String);
}

pub fn get_all_suggestions() -> Vec<Box<Suggestion>> {
    vec![
        Box::new(self::missing_import::MissingImport::default())
    ]
}
