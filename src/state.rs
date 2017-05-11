pub struct State {
    pub working_directory: String,
}

impl State {
    pub fn new() -> State {
        State { working_directory: String::from(".") }
    }
}
