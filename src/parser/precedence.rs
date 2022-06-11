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
        TokenType::Plus => Precedence::Sum,
        TokenType::Minus => Precedence::Sum,
        TokenType::Asterisk => Precedence::Product,
        TokenType::Slash => Precedence::Product,
        TokenType::Eq => Precedence::Equals,
        TokenType::NotEq => Precedence::Equals,
        TokenType::Lt => Precedence::LessGreater,
        TokenType::Gt => Precedence::LessGreater,
        TokenType::LParen => Precedence::Call,
        TokenType::LBracket => Precedence::Index,
        _ => Precedence::Lowest,
    }
}
