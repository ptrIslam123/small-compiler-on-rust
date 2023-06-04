pub mod front {
    use std::collections::HashMap;

    use crate::token;
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

                if self.processOperator(&mut tokens) {
                    continue;
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
            // This code is wrong
            use std::collections::HashMap;
            let keyWords = HashMap::from([
                (String::from("def"), TokenType::Define),
                (String::from("var"), TokenType::Variable),
                (String::from("func"), TokenType::Function),
                (String::from("while"), TokenType::While),
                (String::from("if"), TokenType::If),
                (String::from("else"), TokenType::Else),
                (String::from("elseif"), TokenType::ElseIf),
                (String::from("true"), TokenType::Bool),
                (String::from("false"), TokenType::Bool),
            ]);

            let textSize = self.text.len();
            let mut buffer = String::new();
            let mut pos = 0;
            let mut oldPos = 0;
            let mut i = self.pos;

            while i < textSize {
                let symbol = self.text.chars().nth(i).unwrap();

                for (keyWord, _) in keyWords.iter() {
                    if pos >= keyWord.len() {
                        continue;
                    }

                    if keyWord.chars().nth(pos).unwrap() == symbol {
                        buffer.push(symbol);
                        oldPos = pos;
                        pos += 1;
                        break;
                    }
                }

                if oldPos == pos {
                    break;
                }

                i += 1;
            } 
            
            println!("keybuff={}", buffer);
            match keyWords.get(&buffer) {
                Some(keyTokenType) => {
                    self.pos += buffer.len();
                    Some(Token::new(keyTokenType.clone(), buffer))            
                },
                None => None
            }
        }

        fn processOperator(&mut self, tokens: &mut Vec<Token>) -> bool {
            use std::collections::HashMap;
            let optTable = HashMap::from([
                ('+', Token::new(TokenType::Plus, String::from("+"))),
                ('-', Token::new(TokenType::Minus, String::from("-"))),
                ('*', Token::new(TokenType::Multiply, String::from("*"))),
                ('/', Token::new(TokenType::Divide, String::from("/"))),
                ('(', Token::new(TokenType::LeftBracket, String::from("("))),
                (')', Token::new(TokenType::RightBracket, String::from(")"))),
                ('{', Token::new(TokenType::LeftBrace, String::from("{"))),
                ('}', Token::new(TokenType::RightBrace, String::from("}"))),
                ('=', Token::new(TokenType::Assign, String::from("="))),
                ('<', Token::new(TokenType::Less, String::from("<"))),
                ('>', Token::new(TokenType::More, String::from(">"))),
                ('&', Token::new(TokenType::And, String::from("&"))),
                ('|', Token::new(TokenType::More, String::from("|"))),
                ('!', Token::new(TokenType::NotEq, String::from("!"))),
                (',', Token::new(TokenType::Comma, String::from(","))),
                (';', Token::new(TokenType::Semicolon, String::from(";"))),
            ]);
            
            let textSize = self.text.len();
            let mut tokenBuffer = Vec::new();
            let mut i = 0;
            while self.pos < textSize && i < 2 {
                let symbol = self.text.chars().nth(self.pos).unwrap();
                match optTable.get(&symbol) {
                    Some(token) => {
                        tokenBuffer.push(token);
                        self.pos += 1;
                        i += 1;
                        continue;
                    },
                    None => {
                        break;
                    }
                }
            }

            match tokenBuffer.len() {
                1 => {
                    tokens.push(tokenBuffer[0].clone());
                    true
                }
                2 => {
                    let first = tokenBuffer[0];
                    let second = tokenBuffer[1];
                    if first == optTable.get(&'<').unwrap() && second == optTable.get(&'=').unwrap() {
                        tokens.push(Token::new(TokenType::LessOrEq, String::from("<=")));
                    } else if first == optTable.get(&'>').unwrap() && second == optTable.get(&'=').unwrap() {
                        tokens.push(Token::new(TokenType::MoreOrEq, String::from(">=")));
                    } else if first == optTable.get(&'=').unwrap() && second == optTable.get(&'=').unwrap() {
                        tokens.push(Token::new(TokenType::Eq, String::from("==")));
                    } else if first == optTable.get(&'!').unwrap() && second == optTable.get(&'=').unwrap() {
                        tokens.push(Token::new(TokenType::NotEq, String::from("!=")));
                    } else if first == optTable.get(&'&').unwrap() && second == optTable.get(&'&').unwrap() {
                        tokens.push(Token::new(TokenType::And, String::from("&&")));
                    } else if first == optTable.get(&'|').unwrap() && second == optTable.get(&'|').unwrap() {
                        tokens.push(Token::new(TokenType::Or, String::from("||")));
                    } else {
                        for token in tokenBuffer {
                            tokens.push(token.clone());
                        }
                    }
                    
                    return true;
                },
                _ => false
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