mod errors;
mod array_expr;
mod object_expr;

use crate::ast::{Expression, *};
use crate::tokens::Token;
use std::iter::{Iterator, FromIterator};
use std::slice::IterMut;
use std::vec::Vec;
use crate::ast::json_object::JsonObjectExpression;
use crate::ast::property_assignment::PropertyAssignmentExpression;
use crate::ast::whitespace::WhitespaceExpression;
use crate::ast::comma::CommaExpression;
use std::error::Error;
use std::collections::VecDeque;
use crate::ast::json_array::JsonArrayExpression;
use array_expr::parse_array_expr;
use object_expr::parse_object_expr;

pub type ParseResult = Result<Box<dyn Expression>, Box<dyn Error>>;

pub struct ParseContext {
    pos: usize,
    line: usize,
    tokens: VecDeque<Token>,
    current: Option<Token>,
}

impl ParseContext {
    pub fn new(tokens: Vec<Token>) -> ParseContext {
        ParseContext {
            pos: 0,
            line: 0,
            tokens: VecDeque::from(tokens),
            current: None,
        }
    }
    pub fn advance(&mut self) -> bool {
        if let Some(t) = self.tokens.pop_front() {
            match t {
                Token::Whitespace('\n') => {
                    self.pos = 0;
                    self.line = self.line + 1;
                }
                _ => {
                    self.pos = self.pos + 1;
                }
            }

            self.current = Some(t);
            true
        } else {
            false
        }
    }

    pub fn rewind(&mut self) -> bool {
        if let Some(t) = self.current {
            self.current = None;
            self.tokens.push_front(t);

            true
        } else {
            false
        }
    }
}

pub fn parse(tokens: Vec<Token>) -> ParseResult {
    let context = &mut ParseContext::new(tokens);

    if context.advance() {
        return match context.current.as_ref().unwrap() {
            Token::OpenCurlyBrace => {
                Ok(parse_object_expr(context)?)
            },
            Token::OpenSquareBrace => {
                Ok(Box::new(parse_array_expr(context)))
            },
            t => {
                Err(Box::new(errors::UnexpectedTokenError::new(context.line, context.pos, *t)))
            }
        };
    } else {
        Err(Box::new(errors::UnexpectedEndOfInputError::new(context.line, context.pos)))
    }
}

// pub fn parse(tokens: &mut Vec<Token>) -> impl Expression {
//     let iter= &mut tokens.iter_mut();
//     if iter.len() < 1 {
//         panic!("No tokens to parse");
//     }
//
//     let first_token = iter.nth(0).expect("Should be at least 1");
//
//     let exp = match first_token {
//         Token::OpenCurlyBrace => parse_object_expression(iter),
//         t => panic!(format!("Unexpected token at position 0: {:?}. Expected {{ or [", t))
//     };
//
//     exp
// }
//
// fn parse_object_expression(iter: &mut IterMut<Token>) -> impl Expression {
//     let mut obj_expr = JsonObjectExpression::new();
//
//     loop {
//         if let Some(t) = iter.next() {
//             match t {
//                 Token::Whitespace(c) => {
//                     let ws: Box<dyn Expression> = Box::new(WhitespaceExpression::new(*c));
//                     obj_expr.add_expr(ws);
//                 },
//                  Token::Quote => {
//                      let q: Box<dyn Expression> = Box::new(parse_property_expression(iter));
//                      obj_expr.add_expr(q);
//                  },
//                 Token::Comma => {
//                     let comm: Box<dyn Expression> = Box::new(CommaExpression::new());
//                     obj_expr.add_expr( comm);
//                 },
//                 Token::CloseCurlyBrace => break,
//                 Token::Unknown(c) => panic!(format!("Unexpected token at position : {:?}", c)),
//                 _ => {
//                     println!("{:?}", obj_expr);
//                     unimplemented!()
//                 }
//             }
//         } else {
//             break;
//         }
//     }
//
//     obj_expr
// }
//
// fn parse_property_expression(iter: &mut IterMut<Token>) -> impl Expression {
//     let mut name = String::new();
//
//     loop {
//         if let Some(t) = iter.next() {
//             match t {
//                 Token::Character(c) => {
//                     name = format!("{}{}", name, c);
//                 },
//                 Token::Quote => {
//                     let n = iter.next();
//                     if let Some(token) = n {
//                         match token {
//                             Token::Colon => break,
//                             u => panic!("Unexpected token {:?}", u)
//                         }
//                     } else {
//                         panic!("Unexpected end of input");
//                     }
//                 },
//                 u => panic!("Unexpected token {:?} at pos . Expected [A-Z|a-z]", u)
//             }
//         }
//     }
//
//     loop {
//         if let Some(t) = iter.next() {
//             match t {
//                 Token::Whitespace(_) => {
//                     continue
//                 },
//                 Token::Quote => {
//                     let mut s = String::new();
//                     while let Some(token) = peekable.next() {
//                         match token {
//                             Token::Character(c) => {
//                                 s = format!("{}{}", s, c);
//                             },
//                             Token::Quote => {
//                                 return PropertyAssignmentExpression::new(name, JsonValue::String(s.clone()))
//                             },
//                             c => panic!("Unexpected token {:?} at pos .", c)
//                         }
//                     }
//                 },
//                 Token::Digit(d) => {
//                     let digits = &mut vec![d];
//
//                     while let Some(token) = iter.next() {
//                         match token {
//                             Token::Digit(i) => digits.push(i),
//                             Token::Comma | Token::CloseCurlyBrace => {
//                                 let base: i64 = 10;
//                                 let k = digits.len() - 1;
//                                 let mut result:i64 = 0;
//                                 for (i, j) in digits.iter().enumerate() {
//                                     let p: i64 = (k - i) as i64;
//                                     let val: i64 = ((**j as i64) * base.pow(p as u32)) as i64;
//                                     result = result + val;
//                                 }
//                                 return PropertyAssignmentExpression::new(name, JsonValue::Number(result));
//                             },
//                             u => panic!("Unexpected token {:?} at pos . Expected digit ',' or '}}'", u)
//                         }
//                     }
//                     panic!("Unexpected end of input");
//                 },
//                 Token::Character(c) => {
//                     let mut chars = vec![c];
//
//                     while let Some(token) = iter.next() {
//                         match token {
//                             Token::Character(b) => chars.push(*b),
//                             Token::Comma | Token::CloseCurlyBrace => {
//                                 let result= String::from_iter(chars.iter());
//
//                                 let result = match result.as_str() {
//                                     "true" => true,
//                                     "false" => false,
//                                     _ => panic!("Unexpected value {:?}. Expected 'true' or 'false'")
//                                 };
//
//                                 return PropertyAssignmentExpression::new(name, JsonValue::Boolean(result));
//                             },
//                             u => panic!("unexpected token {:?} at pos . Expected character or ',' or '}}'", u)
//                         }
//                     }
//                     panic!("Unexpected end of input");
//                 },
//                 other => unimplemented!("{:?}", other)
//             }
//         }
//     }
// }
//
//

#[cfg(test)]
mod parsecontext_tests {
    use super::*;

    #[test]
    fn parsecontext_does_not_advance_empty_tokens() {
        let mut pc = ParseContext::new(Vec::new());

        let result = pc.advance();

        assert!(!result);
        assert_eq!(pc.current, None)
    }
}

#[cfg(test)]
mod parse_tests {
    use crate::tokens::Token;
    use super::*;
    use crate::parsing::errors::{UnexpectedEndOfInputError, UnexpectedTokenError};
    use std::borrow::Borrow;
    use std::any::TypeId;

    macro_rules! assert_err {
        ($e:ident, $t:ty) => {{
            println!("{}", $e.is::<$t>());
            assert!($e.is::<$t>());
        }}
    }

    // macro_rules! assert_ok {
    //     ($e:ident)
    // }

    macro_rules! err_test {
        ($name:ident, $v:expr, $t:ty) => {
            #[test]
            fn $name() {
                let tokens = $v;

                match parse(tokens) {
                    Err(e) => assert_err!(e, $t),
                    _ => assert!(false)
                }
            }
        }
    }

    err_test!(parse_returns_err_empty_tokens, Vec::new(), UnexpectedEndOfInputError);
    err_test!(parse_returns_err_if_start_close_curly_brace, vec![Token::CloseCurlyBrace], UnexpectedTokenError);
    err_test!(parse_returns_err_if_start_close_square_brace, vec![Token::CloseSquareBrace], UnexpectedTokenError);
    err_test!(parse_returns_err_if_start_open_parens, vec![Token::OpenParenthesis], UnexpectedTokenError);
    err_test!(parse_returns_err_if_start_close_parens, vec![Token::CloseParenthesis], UnexpectedTokenError);
    err_test!(parse_returns_err_if_start_quote, vec![Token::Quote], UnexpectedTokenError);
    err_test!(parse_returns_err_if_start_comma, vec![Token::Comma], UnexpectedTokenError);
    err_test!(parse_returns_err_if_start_colon, vec![Token::Colon], UnexpectedTokenError);

    // #[test]
    // fn parse_returns_err_empty_tokens() {
    //     let tokens = Vec::new();
    //
    //     match parse(tokens) {
    //         Err(e) => assert_err!(e, UnexpectedEndOfInputError),
    //         _ => assert!(false)
    //     };
    // }

    #[test]
    fn parse_returns_err_if_not_start_object_or_array() {
        let tokens = vec![Token::Whitespace(' ')];

        match parse(tokens) {
            Err(e) => assert_err!(e, UnexpectedTokenError),
            _ => assert!(false)
        }
    }
}