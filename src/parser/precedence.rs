use crate::lexer::token::TokenType;

#[derive(PartialEq, Eq, PartialOrd, Ord, Debug)]
pub enum Precedence {
    Lowest,
    Equals,
    LessGreater,
    Sum,
    Product,
    Prefix,
    Call,
    Index,
}

pub fn get_operator_precedence(operator_type: TokenType) -> Precedence {
    match operator_type {
        TokenType::PLUS => Precedence::Sum,
        TokenType::MINUS => Precedence::Sum,
        TokenType::ASTERISK => Precedence::Product,
        TokenType::SLASH => Precedence::Product,
        TokenType::EQ => Precedence::Equals,
        TokenType::NOTEQ => Precedence::Equals,
        TokenType::LT => Precedence::LessGreater,
        TokenType::GT => Precedence::LessGreater,
        TokenType::LPAREN => Precedence::Call,
        TokenType::LBRACKET => Precedence::Index,
        _ => Precedence::Lowest,
    }
}
