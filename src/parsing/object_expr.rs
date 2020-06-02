use super::ParseContext;
use crate::ast::{
    Expression,
    JsonValue,
    comma::CommaExpression,
    json_object::JsonObjectExpression,
    property_assignment::PropertyAssignmentExpression,
};
use crate::tokens::Token;
use super::errors::UnexpectedTokenError;
use crate::parsing::{ParseResult, parse};
use crate::ast::name::NameExpression;
use crate::ast::value::ValueExpression;
use crate::parsing::errors::UnexpectedEndOfInputError;

pub fn parse_object_expr(ctx: &mut ParseContext) -> ParseResult {
    let mut jo =  JsonObjectExpression::new();

    loop {
        if ctx.advance() {
            match ctx.current.as_ref().unwrap() {
                Token::Whitespace(_) => continue,
                Token::Quote => jo.add_expr(parse_assignment_expr(ctx)?),
                Token::Comma => jo.add_expr(Box::new(CommaExpression::new())),
                Token::CloseCurlyBrace => break,
                t => return Err(Box::new(UnexpectedTokenError::new(ctx.line, ctx.pos, t.clone())))
            };
        } else {

        }
    }

    Ok(Box::new(jo))
}

pub fn parse_assignment_expr(ctx: &mut ParseContext) -> ParseResult {
    let name = parse_name(ctx)?;
    let value = parse_value(ctx)?;

    Ok(Box::new(PropertyAssignmentExpression::new(name, value)))
}

pub fn parse_name(ctx: &mut ParseContext) -> ParseResult {
    let mut name = String::new();

    let mut name_ended = false;
    loop {
        if ctx.advance() {
            match ctx.current.as_ref().unwrap() {
                Token::Character(c) => name.push(*c),
                Token::Digit(d) => {
                    if name.len() > 0 {
                        name.push_str(format!("{}", *d).as_str())
                    } else {
                        return Err(Box::new(UnexpectedTokenError::new(ctx.line, ctx.pos, Token::Digit(*d))));
                    }
                },
                Token::Quote => {
                    name_ended = true;
                },
                Token::Colon => break,
                Token::Whitespace(s) => {
                    if !name_ended {
                        return Err(Box::new(UnexpectedTokenError::new(ctx.line, ctx.pos, Token::Whitespace(*s))));
                    }
                },
                t => return Err(Box::new(UnexpectedTokenError::new(ctx.line, ctx.pos, t.clone()))),
            }
        } else {
            return Err(Box::new(UnexpectedEndOfInputError::new(ctx.line, ctx.pos)));
        }
    }

    Ok(Box::new(NameExpression::new(name)))
}

pub fn parse_value(ctx: &mut ParseContext) -> ParseResult {
    loop {
        if ctx.advance() {
            match ctx.current.as_ref().unwrap() {
                Token::Whitespace(c) => continue,
                Token::Quote => return Ok(parse_string(ctx)?),
                Token::Digit(d) => return Ok(parse_number(ctx, *d, false)?),
                Token::NegativeSign => {
                    return if ctx.advance() {
                        match ctx.current.as_ref().unwrap() {
                            Token::Digit(d) => Ok(parse_number(ctx, *d, true)?),
                            t => Err(Box::new(UnexpectedTokenError::new(ctx.line, ctx.pos, t.clone())))
                        }
                    } else {
                        Err(Box::new(UnexpectedEndOfInputError::new(ctx.line, ctx.pos)))
                    }
                }
                Token::Character('t') | Token::Character('f') => return Ok(parse_bool(ctx)?),
                t => return Err(Box::new(UnexpectedTokenError::new(ctx.line, ctx.pos, t.clone())))
            }
        } else {
            return Err(Box::new(UnexpectedEndOfInputError::new(ctx.line, ctx.pos)))
        }
    }
}

pub fn parse_string(ctx: &mut ParseContext) -> ParseResult {
    let mut s = String::new();

    loop {
        if ctx.advance() {
            match ctx.current.as_ref().unwrap() {
                Token::Whitespace(ws) => s.push(*ws),
                Token::Character(c) => s.push(*c),
                Token::Digit(d) => s.push_str(format!("{}", *d).as_str()),
                Token::Quote => return Ok(Box::new(ValueExpression::new(JsonValue::String(s)))),
                t => return Err(Box::new(UnexpectedTokenError::new(ctx.pos, ctx.line, t.clone())))
            }
        } else {
            return Err(Box::new(UnexpectedEndOfInputError::new(ctx.line, ctx.pos)));
        }
    }
}

pub fn parse_number(ctx: &mut ParseContext, first_digit: u8, is_neg: bool) -> ParseResult {
    let mut num: i64 = first_digit as i64;

    loop {
        if ctx.advance() {
            match ctx.current.as_ref().unwrap() {
                Token::Digit(d) => {
                    num = (num * 10) + (*d as i64);
                },
                Token::Comma | Token::CloseCurlyBrace => {
                    if !ctx.rewind() {
                        panic!("We were unable to back up the token context");
                    }

                    if is_neg {
                        num = num * -1;
                    }

                    return Ok(Box::new(ValueExpression::new(JsonValue::Number(num))))
                }
                t => return Err(Box::new(UnexpectedTokenError::new(ctx.line, ctx.pos, t.clone())))
            }
        } else {
            return Err(Box::new(UnexpectedEndOfInputError::new(ctx.line, ctx.pos)));
        }
    }
}

pub fn parse_bool(ctx: &mut ParseContext) -> ParseResult {
    unimplemented!()
}