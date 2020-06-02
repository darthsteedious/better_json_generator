mod ast;
mod parsing;
mod tags;
mod tokens;

use ast::{
    Expression,
    visitor::{ExpressionVisitor, JsonExpressionVisitor}};
use parsing::parse;
use tokens::process_str;
use std::collections::VecDeque;

fn main() {
    let mut tokens = process_str("{\"foo\": \"bar\", \"baz\": -123, \"bing\": 5}");
    let mut result = parse(tokens);

    match result {
        Ok(mut expr) => {
            let  v = &mut JsonExpressionVisitor::new();
            expr.accept(v);

            println!("{}", v.get_json());
        },
        Err(e) => panic!("Error trying to parse tokens. {}", e)
    }
}
