use crate::{commands::Command, lexer::Lexer, tokens::Token};

pub struct Parser {
    tokens: Vec<Token>,
    position: usize,
}

impl Parser {
    pub fn new(tokens: Vec<Token>) -> Self {
        Self {
            tokens,
            position: 0,
        }
    }

    pub fn parse(&mut self) -> Vec<Command> {
        let mut commands = vec![];
        while let Some(command) = self.parse_token() {
            commands.push(command);
        }
        commands
    }

    pub fn parse_token(&mut self) -> Option<Command> {
        let token = self.curr_token()?;
        match token {
            Token::Ingredient => self.parse_ingredient(),
            Token::Taste => self.parse_taste(),
            Token::Layer => self.parse_layer(),
            Token::Simmer => self.parse_simmer(),
            Token::Cook => self.parse_cook(),
            Token::Serve => self.parse_serve(),
            Token::Plate => self.parse_plate(),
            Token::Recipe => self.parse_recipe(),
            _ => None,
        }
    }

    pub fn parse_ingredient(&mut self) -> Option<Command> {
        self.expect_ingredient()?;
        let identifier = self.expect_identifier()?;
        self.expect_token(Token::Is)?;
        let value = self.expect_value()?;
        Some(Command::Ingredient { identifier, value })
    }

    pub fn parse_taste(&mut self) -> Option<Command> {
        self.expect_token(Token::Taste)?;
        let identifier_1 = self.expect_identifier()?;
        let comparison = self.expect_comparison()?;
        let identifier_2 = self.expect_identifier()?;
        let block = self.expect_string_literal()?;
        if let Token::StringLiteral(string) = block {
            let mut lexer = Lexer::new(&string);
            let tokens = lexer.lex();
            return Some(Command::Taste {
                identifier_1,
                identifier_2,
                comparison,
                tokens,
            });
        }
        None
    }

    pub fn parse_layer(&mut self) -> Option<Command> {
        self.expect_token(Token::Layer)?;
        let identifier_1 = self.expect_identifier()?;
        let comparison = self.expect_comparison()?;
        let identifier_2 = self.expect_identifier()?;
        let block = self.expect_string_literal()?;
        if let Token::StringLiteral(string) = block {
            let mut lexer = Lexer::new(&string);
            let tokens = lexer.lex();
            return Some(Command::Layer {
                identifier_1,
                identifier_2,
                comparison,
                tokens,
            });
        }
        None
    }

    pub fn parse_simmer(&mut self) -> Option<Command> {
        self.expect_token(Token::Simmer)?;
        let identifier = self.expect_identifier()?;
        self.expect_token(Token::In)?;
        let array_identifier = self.expect_identifier()?;
        let tokens = self.expect_block()?;
        let mut parser = Parser::new(tokens);
        let commands = parser.parse();
        Some(Command::Simmer {
            identifier,
            array_identifier,
            commands,
        })
    }

    pub fn parse_cook(&mut self) -> Option<Command> {
        self.expect_token(Token::Cook)?;
        let block = self.expect_string_literal()?;
        if let Token::StringLiteral(string) = block {
            let mut lexer = Lexer::new(&string);
            let tokens = lexer.lex();
            return Some(Command::Cook { tokens });
        }
        None
    }

    pub fn parse_serve(&mut self) -> Option<Command> {
        self.expect_token(Token::Serve)?;
        let identifier = self.expect_identifier()?;
        Some(Command::Serve { identifier })
    }

    pub fn parse_plate(&mut self) -> Option<Command> {
        self.expect_token(Token::Plate)?;
        let value = self.next_token()?;
        Some(Command::Plate { value })
    }

    pub fn parse_recipe(&mut self) -> Option<Command> {
        self.expect_token(Token::Recipe)?;
        let params = self.expect_params()?;
        let block = self.expect_string_literal()?;
        if let Token::StringLiteral(string) = block {
            let mut lexer = Lexer::new(&string);
            let tokens = lexer.lex();
            return Some(Command::Recipe { params, tokens });
        }
        None
    }

    pub fn expect_block(&mut self) -> Option<Vec<Token>> {
        if let Some(Token::Block(tokens)) = self.next_token() {
            Some(tokens)
        } else {
            None
        }
    }

    pub fn expect_ingredient(&mut self) -> Option<Token> {
        if let Some(Token::Ingredient) = self.next_token() {
            Some(Token::Ingredient)
        } else {
            None
        }
    }

    pub fn expect_identifier(&mut self) -> Option<String> {
        if let Some(Token::Identifier(identifier)) = self.next_token() {
            Some(identifier)
        } else {
            None
        }
    }

    pub fn expect_value(&mut self) -> Option<Token> {
        let token = self.next_token()?;
        match token {
            Token::ArrayNumber(array) => Some(Token::ArrayNumber(array)),
            Token::ArrayString(array) => Some(Token::ArrayString(array)),
            Token::Number(number) => Some(Token::Number(number)),
            Token::StringLiteral(string) => Some(Token::StringLiteral(string)),
            _ => None,
        }
    }

    pub fn expect_comparison(&mut self) -> Option<Token> {
        let token = self.next_token()?;
        match token {
            Token::Hotter => Some(Token::Hotter),
            Token::Cooler => Some(Token::Cooler),
            Token::Same => Some(Token::Same),
            Token::HotOrSame => Some(Token::HotOrSame),
            Token::CoolOrSame => Some(Token::CoolOrSame),
            Token::NotSame => Some(Token::NotSame),
            _ => None,
        }
    }

    pub fn expect_token(&mut self, expected_token: Token) -> Option<Token> {
        if let Some(token) = self.next_token() {
            if expected_token == token {
                return Some(token);
            }
            None
        } else {
            None
        }
    }

    pub fn expect_string_literal(&mut self) -> Option<Token> {
        if let Some(Token::StringLiteral(string)) = self.next_token() {
            return Some(Token::StringLiteral(string));
        }
        None
    }

    pub fn expect_params(&mut self) -> Option<Vec<String>> {
        if let Some(Token::ArrayString(array_string)) = self.next_token() {
            return Some(array_string);
        }
        None
    }

    pub fn curr_token(&self) -> Option<Token> {
        self.tokens.get(self.position).cloned()
    }
    pub fn next_token(&mut self) -> Option<Token> {
        if self.tokens.len() < self.position + 1 {
            return None;
        }
        let token = self.curr_token();
        self.advance();
        token
    }
    pub fn advance(&mut self) {
        self.position += 1
    }
}
