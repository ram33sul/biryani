use crate::{
    commands::Command,
    state::{State, Value},
    tokens::Token,
};

pub struct Interpreter {
    state: State,
}

impl Interpreter {
    pub fn new() -> Self {
        Self {
            state: State::new(),
        }
    }

    pub fn execute(&mut self, commands: Vec<Command>) {
        for command in commands {
            match command {
                Command::Ingredient { identifier, value } => {
                    self.execute_ingredient(identifier, value)
                }
                Command::Plate { value } => self.execute_plate(value),
                Command::Simmer {
                    identifier,
                    array_identifier,
                    commands,
                } => self.execute_simmer(identifier, array_identifier, commands),
                _ => panic!("Invalid command"),
            };
        }
    }

    fn execute_ingredient(&mut self, identifier: String, value: Token) -> Option<Value> {
        match value {
            Token::ArrayNumber(array) => self
                .state
                .add_variable(&identifier, Value::ArrayNumber(array)),
            Token::ArrayString(array) => self
                .state
                .add_variable(&identifier, Value::ArrayString(array)),
            Token::Number(number) => self.state.add_variable(&identifier, Value::Number(number)),
            Token::StringLiteral(string) => {
                self.state.add_variable(&identifier, Value::String(string))
            }
            _ => panic!("Invalid value for identifier: {}", identifier),
        }
    }

    fn execute_simmer(
        &mut self,
        identifier: String,
        array_identifier: String,
        commands: Vec<Command>,
    ) -> Option<Value> {
        let value = self.state.get_variable(array_identifier)?;
        match value {
            Value::ArrayNumber(array) => {
                for val in array {
                    self.state.add_variable(&identifier, Value::Number(val));
                    self.execute(commands.clone());
                }
            }
            Value::ArrayString(array) => {
                for val in array {
                    self.state.add_variable(&identifier, Value::String(val));
                    self.execute(commands.clone());
                }
            }
            _ => panic!("for loop is not iteratable"),
        }
        None
    }

    fn execute_plate(&mut self, token: Token) -> Option<Value> {
        match token {
            Token::Identifier(identifier) => {
                let value = self.state.get_variable(identifier)?;
                match value {
                    Value::ArrayNumber(array) => {
                        println!("{:?}", array);
                    }
                    Value::ArrayString(array) => {
                        println!("{:?}", array);
                    }
                    Value::Number(number) => {
                        println!("{}", number);
                    }
                    Value::String(string) => {
                        println!("{}", string);
                    }
                }
            }
            Token::StringLiteral(str) => {
                println!("{}", str);
            }
            Token::Number(number) => {
                println!("{}", number);
            }
            Token::ArrayNumber(array) => {
                println!("{:?}", array);
            }
            Token::ArrayString(array) => {
                println!("{:?}", array);
            }
            _ => panic!("Invalid value passed for Plate"),
        };
        None
    }
}
