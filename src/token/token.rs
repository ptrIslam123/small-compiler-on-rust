pub mod front {

    #[derive(Clone, PartialEq)]
    pub enum TokenType {
        Undefined,
        Number,                             // 1,2,23, 34, ..
        Char,                               // 'h', '#', ...
        Bool,                               // true, false
        Str,                                // "Hello", "world", ...

        Plus,                               // +
        Minus,                              // -
        Multiply,                           // *
        Divide,                             // /

        Assign,                             // =
        Less,                               // <
        LessOrEq,                           // <=
        More,                               // >
        MoreOrEq,                           // >=
        NotEq,                              // !
        Eq,                                 // ==
        And,                                // &&
        Or,                                 // ||
        Comma,                              // ,
        Semicolon,                          // ;
        Word,                               // myVariableName, myFunctionName1, myFuncName23, myArray_Name34, ... 

        LeftBrace,                          // {
        RightBrace,                         // }
        LeftBracket,                        // (
        RightBracket,                       // )

        Define,                             // key word: use to define {variable, function, array and etc}
        Variable,                           // key word: define variable
        Function,                           // key word: define function
        While,                              // while
        If,                                 // if
        Else,                               // else
        ElseIf,                             // else if     
    }
    
    pub fn TokenTypeToStr(tokenType: TokenType) -> String {
        match tokenType {
            TokenType::Number => String::from("Number"),
            TokenType::Char => String::from("Char"),
            TokenType::Bool => String::from("Bool"),
            TokenType::Str => String::from("String"),
            TokenType::Plus => String::from("Plus"),
            TokenType::Minus => String::from("Minus"),
            TokenType::Multiply => String::from("Multiply"),
            TokenType::Divide => String::from("Divide"),
            TokenType::Word => String::from("Word"),
            TokenType::Comma => String::from("Comma"),
            TokenType::Assign => String::from("Assign"),
            TokenType::Semicolon => String::from("Semicolon"),
            TokenType::LeftBracket => String::from("LeftBracket"),
            TokenType::RightBracket => String::from("RightBracket"),
            TokenType::LeftBrace => String::from("LeftBrace"),
            TokenType::RightBrace => String::from("RightBrace"),
            TokenType::Less => String::from("Less"),
            TokenType::LessOrEq => String::from("LessOrEq"),
            TokenType::More => String::from("More"),
            TokenType::MoreOrEq => String::from("MoreOrEq"),
            TokenType::NotEq => String::from("NotEq"),
            TokenType::Eq => String::from("Eq"),
            TokenType::And => String::from("And"),
            TokenType::Or => String::from("Or"),
            TokenType::Define => String::from("Define"), 
            TokenType::Variable => String::from("Variable"),
            TokenType::Function => String::from("Function"),
            TokenType::If => String::from("If"),
            TokenType::Else => String::from("Else"),
            TokenType::ElseIf => String::from("ElseIf"),
            TokenType::While => String::from("While"),
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

    impl PartialEq for Token {
        fn eq(&self, other: &Self) -> bool {
            self.tokenType == other.tokenType
        }
    }
    
}
