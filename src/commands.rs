use crate::{state::Value, tokens::Token};

#[derive(Debug, Clone)]
pub enum Command {
    Ingredient {
        identifier: String,
        values: Vec<Token>,
    },
    Taste {
        identifier_1: String,
        comparison: Token,
        identifier_2: String,
        tokens: Vec<Token>,
    },
    TasteAgain {
        condition: bool,
        tokens: Vec<Token>,
    },
    Tasteless {
        tokens: Vec<Token>,
    },
    Layer {
        left_value: Token,
        right_value: Token,
        comparison: Token,
        commands: Vec<Command>,
    },
    Simmer {
        identifier: String,
        array_identifier: String,
        commands: Vec<Command>,
    },
    Cook {
        tokens: Vec<Token>,
    },
    Burnt {
        error: String,
        tokens: Vec<Token>,
    },
    Serve {
        identifier: String,
    },
    Plate {
        value: Token,
    },
    Recipe {
        identifier: String,
        params: Vec<String>,
        commands: Vec<Command>,
    },
    Maths {
        values: Vec<Token>,
    },
}
