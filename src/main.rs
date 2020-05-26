mod ast;
mod parsing;
mod tokens;

use ast::{
    Expression,
    visitor::{ExpressionVisitor, JsonExpressionVisitor}};
use parsing::parse;
use tokens::process_str;

fn main() {
    let mut tokens = process_str("{\"foo\": 1230, \"bar\": true}");

    let mut result = parse(&mut tokens);

    let mut v = JsonExpressionVisitor::new();

    result.accept(&mut v);

    println!("{}", v.get_json());
}
