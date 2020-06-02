use std::error::Error;
use std::fmt::{Display, Formatter, Result};
use crate::tokens::Token;
use std::borrow::Borrow;

#[derive(Debug)]
pub struct UnexpectedTokenError {
    line: usize,
    pos: usize,
    token: Token
}

impl UnexpectedTokenError {
    pub fn new(line: usize, pos: usize, token: Token) -> impl Error {
        UnexpectedTokenError {line, pos, token}
    }
}

impl Error for UnexpectedTokenError {}

impl Display for UnexpectedTokenError {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        write!(f, "Unexpected token {}. Line {}. Position {}", char::from(&self.token), self.line, self.pos)
    }
}

#[derive(Debug)]
pub struct UnexpectedEndOfInputError {
    line: usize,
    pos: usize
}

impl UnexpectedEndOfInputError {
    pub fn new(line: usize, pos: usize) -> impl Error {
        UnexpectedEndOfInputError{line, pos}
    }
}

impl Error for UnexpectedEndOfInputError {}

impl Display for UnexpectedEndOfInputError {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        write!(f, "Unexpected end of input at line {}. pos {}", self.line, self.pos)
    }
}