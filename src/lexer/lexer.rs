pub mod front {
    use crate::token::Token;
    use crate::token::TokenType;

    pub struct Lexer {
        text: String,
        pos:  usize,
    }

    impl Lexer {
        pub fn new(text: String) -> Lexer {
            Lexer{ text: text, pos: 0 }
        }

        pub fn analyze(&mut self) -> Vec<Token> {
            let mut tokens = Vec::new();
            let textSize = self.text.len();
            while self.pos < textSize {
                let symbol = self.text.chars().nth(self.pos).unwrap();

                if symbol.is_whitespace() || symbol == '\n' || symbol == '\r' {
                    self.pos += 1;
                    continue;
                }

                if symbol == '\"' {
                    match self.processString() {
                        Some(token) => {
                            tokens.push(token);
                            continue;
                        }
                        None => {
                            // this is error
                            tokens.push(Token::new(TokenType::Undefined, String::from("Invalid string")));
                            self.pos += 1;
                            continue;
                        }
                    }
                }

                if symbol == '\'' {
                    match self.processChar() {
                        Some(token) => {
                            tokens.push(token);
                            continue
                        }
                        None => {
                            // this is error
                            tokens.push(Token::new(TokenType::Undefined, String::from("Invalid char")));
                            self.pos += 1;
                            continue;
                        }
                    }
                }

                match self.processKeyWord() {
                    Some(token) => {
                        tokens.push(token);
                        continue;
                    }
                    None => {/* It is ok! */}
                }

                match self.processOperator(&symbol) {
                    Some(token) => {
                        tokens.push(token); 
                        self.pos += 1;
                        continue;
                    }
                    None => {/* It is ok! */},
                }

                
                if symbol.is_numeric() {
                    tokens.push(self.processNumeric());
                    continue;
                } else if symbol.is_alphabetic() {
                    tokens.push(self.processWord());
                    continue;
                } else {
                    let mut message = String::from("Compiler detected undefined toke: ");
                    message.push(symbol);
                    tokens.push(Token::new(TokenType::Undefined, message));
                    self.pos += 1;
                    continue;
                }
            }

            tokens
        }

        fn processKeyWord(&mut self) -> Option<Token> {
            None
        }

        fn processOperator(&self, symbol: &char) -> Option<Token> {
            use std::collections::HashMap;
            let optTable = HashMap::from([
                ('+', Token::new(TokenType::Plus, String::from(*symbol))),
                ('-', Token::new(TokenType::Minus, String::from(*symbol))),
                ('*', Token::new(TokenType::Multiply, String::from(*symbol))),
                ('/', Token::new(TokenType::Divide, String::from(*symbol))),
                ('(', Token::new(TokenType::LeftBracket, String::from(*symbol))),
                (')', Token::new(TokenType::RightBracket, String::from(*symbol))),
                ('=', Token::new(TokenType::Eq, String::from(*symbol))),
                (',', Token::new(TokenType::Comma, String::from(*symbol))),
                (';', Token::new(TokenType::Semicolon, String::from(*symbol))),
            ]);

            match optTable.get(symbol) {
                Some(&ref token) => Some(token.clone()),
                None => None,
            }
        }

        fn processWord(&mut self) -> Token {
            let mut buffer = String::new();
            let textSize = self.text.len();
            while self.pos < textSize {
                let symbol = self.text.chars().nth(self.pos).unwrap();
                if symbol.is_alphabetic() || symbol.is_numeric() || symbol == '_' {
                    buffer.push(symbol);
                    self.pos += 1;
                    continue;
                }
                break;
            }

            Token::new(TokenType::Word, buffer)
        }

        fn processString(&mut self) -> Option<Token> {
            self.pos += 1;
            let mut buffer = String::new();
            let textSize = self.text.len();
            while self.pos < textSize {
                let symbol = self.text.chars().nth(self.pos).unwrap();

                if symbol == '\"' {
                    self.pos += 1;
                    return Some(Token::new(TokenType::Str, buffer));
                }

                buffer.push(symbol);
                self.pos += 1;
            }

            None
        }

        fn processChar(&mut self) -> Option<Token> {
            self.pos += 1;
            let mut buffer = String::new();
            let textSize = self.text.len();
            while self.pos < textSize {
                let symbol = self.text.chars().nth(self.pos).unwrap();
                if symbol == '\'' {
                    self.pos += 1;
                    break;
                }

                buffer.push(symbol);
                self.pos += 1;
            }

            if buffer.len() == 1 {
                Some(Token::new(TokenType::Char, buffer))
            } else {
                None
            }
        }

        fn processNumeric(&mut self) -> Token {
            let mut buffer = String::new();
            let textSize = self.text.len();
            for i in self.pos..textSize {
                let symbol = self.text.chars().nth(i).unwrap();
                if symbol.is_numeric() {
                    buffer.push(symbol);
                    self.pos += 1;
                    continue;
                }
                break;
            }

            Token::new(TokenType::Number, buffer)
        }
    }
}