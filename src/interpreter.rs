use core::num;

use crate::{
    commands::Command,
    state::{State, Value},
    tokens::Token,
    utils::MathsOperations,
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
                Command::Ingredient { identifier, values } => {
                    self.execute_ingredient(identifier, values)
                }
                Command::Plate { value } => self.execute_plate(value),
                Command::Layer {
                    left_value,
                    right_value,
                    comparison,
                    commands,
                } => self.execute_layer(left_value, right_value, comparison, commands),
                Command::Simmer {
                    identifier,
                    array_identifier,
                    commands,
                } => self.execute_simmer(identifier, array_identifier, commands),
                Command::Recipe {
                    identifier,
                    params,
                    commands,
                } => self.execute_recipe(identifier, params, commands),
                Command::Maths { values } => self.execute_maths(values),
                _ => panic!("Invalid command"),
            };
        }
    }

    fn execute_maths(&mut self, values: Vec<Token>) -> Option<Value> {
        let mut result = 0;
        let mut operation = &MathsOperations::Plus;
        for val in values.iter() {
            let number = match val {
                Token::Number(num) => Some(*num),
                Token::Identifier(identifier) => {
                    let value = self.state.get_variable(identifier.clone())?;
                    match value {
                        Value::Number(num) => Some(num),
                        _ => panic!("Operand should be number"),
                    }
                }
                _ => None,
            };
            if let Some(num) = number {
                result = match operation {
                    MathsOperations::Plus => result + num,
                    MathsOperations::Minus => result - num,
                    MathsOperations::Multiply => result * num,
                    MathsOperations::Division => result / num,
                    MathsOperations::Mod => result % num,
                }
            } else {
                operation = match val {
                    Token::Maths(oper) => oper,
                    _ => panic!("Invalid operation"),
                }
            }
        }
        Some(Value::Number(result))
    }

    fn execute_ingredient(&mut self, identifier: String, values: Vec<Token>) -> Option<Value> {
        if values.len() > 1 {
            let result = self.execute_maths(values)?;
            return self.state.add_variable(&identifier, result);
        }
        let value = values[0].clone();
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

    fn execute_layer(
        &mut self,
        left_value: Token,
        right_value: Token,
        comparison: Token,
        commands: Vec<Command>,
    ) -> Option<Value> {
        let error_message = "Operator should be number for layer";
        loop {
            let operator_1 = match &left_value {
                &Token::Number(number) => number,
                Token::Identifier(identifier) => {
                    let value = self.state.get_variable(identifier.to_string())?;
                    if let Value::Number(number) = value {
                        number
                    } else {
                        panic!("{}", error_message);
                    }
                }
                _ => panic!("{}", error_message),
            };
            let operator_2 = match &right_value {
                &Token::Number(number) => number,
                Token::Identifier(identifier) => {
                    let value = self.state.get_variable(identifier.to_string())?;
                    if let Value::Number(number) = value {
                        number
                    } else {
                        panic!("{}", error_message)
                    }
                }
                _ => panic!("{}", error_message),
            };
            let comparison = match comparison {
                Token::Hotter => operator_1 > operator_2,
                Token::Cooler => operator_1 < operator_2,
                Token::Same => operator_1 == operator_2,
                Token::HotOrSame => operator_1 >= operator_2,
                Token::CoolOrSame => operator_1 <= operator_2,
                Token::NotSame => operator_1 != operator_2,
                _ => panic!("Invalid comparison operator"),
            };
            if comparison {
                self.execute(commands.clone());
            } else {
                break;
            }
        }
        None
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
                    Value::Function(params, tokens) => {
                        println!("params: {:?}, tokens: {:?}", params, tokens);
                    }
                    Value::Boolean(boolean) => {
                        println!("{}", boolean);
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

    pub fn execute_recipe(
        &mut self,
        identifier: String,
        params: Vec<String>,
        commands: Vec<Command>,
    ) -> Option<Value> {
        self.state
            .add_variable(&identifier, Value::Function(params, commands))
    }
}
