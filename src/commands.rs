use crate::tokens::Token;

#[derive(Debug, Clone)]
pub enum Command {
    Ingredient {
        identifier: String,
        value: Token,
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
        identifier_1: String,
        comparison: Token,
        identifier_2: String,
        tokens: Vec<Token>,
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
        params: Vec<String>,
        tokens: Vec<Token>,
    },
}
