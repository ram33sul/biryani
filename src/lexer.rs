use crate::{tokens::Token, utils::ValueType};

pub struct Lexer {
    input: Vec<char>,
    position: usize,
}

impl Lexer {
    pub fn new(input: &str) -> Self {
        Self {
            input: input.chars().collect(),
            position: 0,
        }
    }

    pub fn lex(&mut self) -> Vec<Token> {
        let mut tokens = vec![];
        while let Some(token) = self.next_token() {
            if token != Token::Space {
                tokens.push(token);
            }
        }
        tokens
    }

    pub fn curr_char(&self) -> Option<char> {
        if self.input.len() <= self.position {
            return None;
        }
        let ch = self.input[self.position];
        Some(ch)
    }

    pub fn next_char(&mut self) -> Option<char> {
        if self.input.len() == self.position {
            return None;
        }
        let ch = self.curr_char();
        self.advance(1);
        ch
    }

    pub fn next_token(&mut self) -> Option<Token> {
        let ch = self.curr_char()?;

        match ch {
            'i' if self.match_keyword("ingredient") => Some(Token::Ingredient),
            't' if self.match_keyword("taste") => Some(Token::Taste),
            't' if self.match_keyword("taste_again") => Some(Token::TasteAgain),
            't' if self.match_keyword("tasteless") => Some(Token::Tasteless),
            'l' if self.match_keyword("layer") => Some(Token::Layer),
            's' if self.match_keyword("simmer") => Some(Token::Simmer),
            'c' if self.match_keyword("cook") => Some(Token::Cook),
            'b' if self.match_keyword("burnt") => Some(Token::Burnt),
            's' if self.match_keyword("serve") => Some(Token::Serve),
            'p' if self.match_keyword("plate") => Some(Token::Plate),
            'r' if self.match_keyword("recipe") => Some(Token::Recipe),
            'o' if self.match_keyword("open") => Some(Token::Fresh),
            'c' if self.match_keyword("close") => Some(Token::Spoiled),
            'm' if self.match_keyword("mix") => Some(Token::Mix),
            's' if self.match_keyword("separate") => Some(Token::Separate),
            'i' if self.match_keyword("is") => Some(Token::Is),
            'i' if self.match_keyword("in") => Some(Token::In),
            '{' => self.parse_block(),
            '[' => self.parse_array(),
            ' ' => self.parse_space(),
            ch if ch.is_digit(10) => self.parse_number(),
            ch if ch.is_alphanumeric() => self.parse_identifier(),
            _ => None,
        }
    }

    pub fn match_keyword(&mut self, keyword: &str) -> bool {
        let mut end_pos = self.position;
        while self.input[end_pos].is_alphabetic() {
            end_pos += 1;
        }
        if end_pos >= self.input.len() {
            end_pos = self.input.len() - 1;
        }
        if self.input[(self.position)..=end_pos - 1] == keyword.chars().collect::<Vec<_>>() {
            self.position = end_pos;
            return true;
        }
        false
    }

    pub fn parse_number(&mut self) -> Option<Token> {
        let mut number = String::new();
        while let Some(ch) = self.next_char() {
            if ch.is_digit(10) {
                number.push(ch);
            } else {
                break;
            }
        }
        if number.is_empty() {
            return None;
        }
        Some(Token::Number(number.parse().unwrap()))
    }

    pub fn parse_identifier(&mut self) -> Option<Token> {
        let mut identifier = String::new();
        while let Some(ch) = self.next_char() {
            if ch.is_alphanumeric() && ch != ' ' {
                identifier.push(ch);
            } else {
                break;
            }
        }
        if identifier.is_empty() {
            return None;
        }
        Some(Token::Identifier(identifier))
    }

    pub fn parse_array(&mut self) -> Option<Token> {
        self.next_char()?;
        let ch = self.next_char()?;
        let value_type = match ch {
            '"' => ValueType::String,
            _ => ValueType::Number,
        };
        self.position -= 1;
        match value_type {
            ValueType::String => self.parse_string_array(),
            ValueType::Number => self.parse_number_array(),
        }
    }

    pub fn parse_number_array(&mut self) -> Option<Token> {
        let mut array = vec![];
        let mut is_negative = false;
        self.position -= 1;
        while let Some(ch) = self.next_char() {
            if ch == ']' {
                break;
            } else if ch == '-' {
                is_negative = true;
            } else if ch.is_digit(10) {
                let mut number_string = String::new();
                if is_negative {
                    number_string.push('-');
                }
                number_string.push(ch);
                array.push(number_string);
            }
            if ch != '-' {
                is_negative = false;
            }
        }
        Some(Token::ArrayNumber(
            array.iter().map(|s| s.parse().unwrap()).collect(),
        ))
    }

    pub fn parse_string_array(&mut self) -> Option<Token> {
        let mut array = vec![];
        let mut is_opened = false;
        let mut curr_str = String::new();
        while let Some(ch) = self.next_char() {
            if ch == ']' {
                break;
            } else if ch == '"' || (is_opened && ch == ',') {
                if is_opened {
                    array.push(curr_str.clone());
                    curr_str.clear();
                }
                is_opened = !is_opened
            } else if is_opened {
                curr_str.push(ch);
            }
        }
        Some(Token::ArrayString(array))
    }

    pub fn parse_block(&mut self) -> Option<Token> {
        let mut block = String::new();
        self.advance(1);
        while let Some(ch) = self.next_char() {
            if ch == '}' {
                break;
            }
            block.push(ch);
        }
        let mut lexer = Lexer::new(&block);
        let tokens = lexer.lex();
        Some(Token::Block(tokens))
    }

    pub fn parse_params(&mut self) -> Option<Token> {
        let mut params = vec![];
        let mut curr_identifier = String::new();
        while let Some(ch) = self.next_char() {
            if ch == ' ' {
                panic!("Spaces are not allowed inside params")
            } else if ch == ',' {
                params.push(curr_identifier);
                curr_identifier = String::new();
            } else if ch == ']' {
                break;
            } else {
                curr_identifier.push(ch);
            }
        }
        Some(Token::ArrayString(params))
    }

    pub fn parse_space(&mut self) -> Option<Token> {
        self.advance(1);
        Some(Token::Space)
    }

    pub fn advance(&mut self, increment: usize) {
        if self.input.len() >= self.position + increment {
            self.position += increment;
        } else {
            panic!("Cannot advance more");
        }
    }
}
