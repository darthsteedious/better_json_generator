use crate::ast::{Expression, *};
use crate::tokens::Token;
use std::iter::{Iterator, FromIterator};
use std::slice::IterMut;
use std::vec::Vec;
use crate::ast::json_object::JsonObjectExpression;
use crate::ast::property_assignment::PropertyAssignmentExpression;
use crate::ast::whitespace::WhitespaceExpression;
use crate::ast::comma::CommaExpression;

pub fn parse(tokens: &mut Vec<Token>) -> impl Expression {
    let iter= &mut tokens.iter_mut();
    if iter.len() < 1 {
        panic!("No tokens to parse");
    }

    let first_token = iter.nth(0).expect("Should be at least 1");

    let exp = match first_token {
        Token::OpenCurlyBrace => parse_object_expression(iter),
        t => panic!(format!("Unexpected token at position 0: {:?}. Expected {{ or [", t))
    };

    exp
}

fn parse_object_expression(iter: &mut IterMut<Token>) -> impl Expression {
    let mut obj_expr = JsonObjectExpression::new();

    loop {
        if let Some(t) = iter.next() {
            match t {
                Token::Whitespace(c) => {
                    let ws: Box<dyn Expression> = Box::new(WhitespaceExpression::new(*c));
                    obj_expr.add_expr(ws);
                },
                 Token::Quote => {
                     let q: Box<dyn Expression> = Box::new(parse_property_expression(iter));
                     obj_expr.add_expr(q);
                 },
                Token::Comma => {
                    let comm: Box<dyn Expression> = Box::new(CommaExpression::new());
                    obj_expr.add_expr( comm);
                },
                Token::CloseCurlyBrace => break,
                Token::Unknown(c) => panic!(format!("Unexpected token at position : {:?}", c)),
                _ => {
                    println!("{:?}", obj_expr);
                    unimplemented!()
                }
            }
        } else {
            break;
        }
    }

    obj_expr
}

fn parse_property_expression(iter: &mut IterMut<Token>) -> impl Expression {
    let mut name = String::new();

    loop {
        if let Some(t) = iter.next() {
            match t {
                Token::Character(c) => {
                    name = format!("{}{}", name, c);
                },
                Token::Quote => {
                    let n = iter.next();
                    if let Some(token) = n {
                        match token {
                            Token::Colon => break,
                            u => panic!("Unexpected token {:?}", u)
                        }
                    } else {
                        panic!("Unexpected end of input");
                    }
                },
                u => panic!("Unexpected token {:?} at pos . Expected [A-Z|a-z]", u)
            }
        }
    }

    loop {
        let mut peekable = iter.into_iter().peekable();
        if let Some(t) = peekable.peek() {
            match t {
                Token::Whitespace(_) => {
                    peekable.next();
                    continue
                },
                Token::Quote => {
                    peekable.next();
                    let mut s = String::new();
                    while let Some(token) = peekable.next() {
                        match token {
                            Token::Character(c) => {
                                s = format!("{}{}", s, c);
                            },
                            Token::Quote => {
                                return PropertyAssignmentExpression::new(name, JsonValue::String(s.clone()))
                            },
                            c => panic!("Unexpected token {:?} at pos .", c)
                        }
                    }
                },
                Token::Digit(d) => {
                    let digits = &mut vec![d];

                    while let Some(token) = iter.next() {
                        match token {
                            Token::Digit(i) => digits.push(i),
                            Token::Comma | Token::CloseCurlyBrace => {
                                let base: i64 = 10;
                                let k = digits.len() - 1;
                                let mut result:i64 = 0;
                                for (i, j) in digits.iter().enumerate() {
                                    let p: i64 = (k - i) as i64;
                                    let val: i64 = ((**j as i64) * base.pow(p as u32)) as i64;
                                    result = result + val;
                                }
                                return PropertyAssignmentExpression::new(name, JsonValue::Number(result));
                            },
                            u => panic!("Unexpected token {:?} at pos . Expected digit ',' or '}}'", u)
                        }
                    }
                    panic!("Unexpected end of input");
                },
                Token::Character(c) => {
                    let mut chars = vec![*c];

                    while let Some(token) = peekable.next() {
                        match token {
                            Token::Character(b) => chars.push(*b),
                            Token::Comma | Token::CloseCurlyBrace => {
                                let result= String::from_iter(chars.iter());

                                let result = match result.as_str() {
                                    "true" => true,
                                    "false" => false,
                                    _ => panic!("Unexpected value {:?}. Expected 'true' or 'false'")
                                };

                                return PropertyAssignmentExpression::new(name, JsonValue::Boolean(result));
                            },
                            u => panic!("unexpected token {:?} at pos . Expected character or ',' or '}}'", u)
                        }
                    }
                    panic!("Unexpected end of input");
                },
                other => unimplemented!("{:?}", other)
            }
        }
    }
}


