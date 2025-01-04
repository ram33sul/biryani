use std::collections::HashMap;

pub struct State {
    pub variables: HashMap<String, Value>,
}

impl State {
    pub fn new() -> Self {
        Self {
            variables: HashMap::new(),
        }
    }

    pub fn add_variable(&mut self, name: &String, value: Value) -> Option<Value> {
        self.variables.insert(name.to_string(), value)
    }

    pub fn get_variable(&mut self, name: String) -> Option<Value> {
        self.variables.get(&name).cloned()
    }
}

#[derive(Clone, Debug)]
pub enum Value {
    String(String),
    Number(isize),
    ArrayString(Vec<String>),
    ArrayNumber(Vec<isize>),
}
