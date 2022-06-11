#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum TokenType {
    Illegal,
    EOF,

    // identifiers, literals
    Ident,
    Int,
    String,

    // operators
    Assign,
    Plus,
    Minus,
    Bang,
    Asterisk,
    Slash,

    Eq,
    NotEq,
    Lt,
    Gt,

    // delimiters
    Comma,
    Semicolon,
    Colon,

    LParen,
    RParen,
    LBrace,
    RBrace,
    LBracket,
    RBracket,

    // keywords
    Function,
    Let,
    True,
    False,
    If,
    Else,
    Return,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Token {
    pub tok_type: TokenType,
    pub literal: String,
}

impl Token {
    pub fn new(tok_type: TokenType, literal: &str) -> Self {
        Self {
            tok_type,
            literal: String::from(literal),
        }
    }

    pub fn new_from_char(tok_type: TokenType, literal: char) -> Self {
        Self {
            tok_type,
            literal: literal.to_string(),
        }
    }

    pub fn lookup_ident_type(literal: &str) -> TokenType {
        return match literal {
            "fn" => TokenType::Function,
            "let" => TokenType::Let,
            "true" => TokenType::True,
            "false" => TokenType::False,
            "if" => TokenType::If,
            "else" => TokenType::Else,
            "return" => TokenType::Return,
            _ => TokenType::Ident,
        };
    }
}

impl ToString for TokenType {
    fn to_string(&self) -> String {
        match self {
            TokenType::Illegal => "ILLEGAL".to_string(),
            TokenType::EOF => "EOF".to_string(),
            TokenType::Ident => "IDENT".to_string(),
            TokenType::Int => "INT".to_string(),
            TokenType::String => "STRING".to_string(),
            TokenType::Assign => "=".to_string(),
            TokenType::Plus => "+".to_string(),
            TokenType::Minus => "-".to_string(),
            TokenType::Bang => "!".to_string(),
            TokenType::Asterisk => "*".to_string(),
            TokenType::Slash => "/".to_string(),
            TokenType::Eq => "==".to_string(),
            TokenType::NotEq => "!=".to_string(),
            TokenType::Lt => "<".to_string(),
            TokenType::Gt => ">".to_string(),
            TokenType::Comma => ",".to_string(),
            TokenType::Semicolon => ";".to_string(),
            TokenType::Colon => ":".to_string(),
            TokenType::LParen => "(".to_string(),
            TokenType::RParen => ")".to_string(),
            TokenType::LBrace => "{{".to_string(),
            TokenType::RBrace => "}}".to_string(),
            TokenType::LBracket => "[".to_string(),
            TokenType::RBracket => "]".to_string(),
            TokenType::Function => "FUNCTION".to_string(),
            TokenType::Let => "LET".to_string(),
            TokenType::True => "TRUE".to_string(),
            TokenType::False => "FALSE".to_string(),
            TokenType::If => "IF".to_string(),
            TokenType::Else => "ELSE".to_string(),
            TokenType::Return => "RETURN".to_string(),
        }
    }
}
