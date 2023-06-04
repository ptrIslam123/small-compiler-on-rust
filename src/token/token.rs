pub mod front {

    #[derive(Clone)]
    pub enum TokenType {
        Undefined,
        Number,
        Char,
        Boolean,
        Str,

        Plus,
        Minus,
        Multiply,
        Divide,

        Eq,
        Comma,
        Semicolon,
        Word,

        LeftBracket,
        RightBracket,
    }
    
    pub fn TokenTypeToStr(tokenType: TokenType) -> String {
        match tokenType {
            TokenType::Number => String::from("Number"),
            TokenType::Char => String::from("Char"),
            TokenType::Boolean => String::from("Boolean"),
            TokenType::Str => String::from("String"),
            TokenType::Plus => String::from("Plus"),
            TokenType::Minus => String::from("Minus"),
            TokenType::Multiply => String::from("Multiply"),
            TokenType::Divide => String::from("Divide"),
            TokenType::Word => String::from("Word"),
            TokenType::Comma => String::from("Comma"),
            TokenType::Eq => String::from("Eq"),
            TokenType::Semicolon => String::from("Semicolon"),
            TokenType::LeftBracket => String::from("LeftBracket"),
            TokenType::RightBracket => String::from("RightBracket"),
            _ => String::from("Undefined"),
        }
    }
    
    #[derive(Clone)]
    pub struct Token {
        tokenType: TokenType,
        tokenValue: String,
    }
    
    impl Token {
        pub fn new(tokenType: TokenType, tokenValue: String) -> Token {
            Token {tokenType: tokenType, tokenValue: tokenValue}
        }

        pub fn getValue(&self) -> String {
            self.tokenValue.clone()
        }

        pub fn getType(&self) -> TokenType {
            self.tokenType.clone()
        }
    }
    
}
