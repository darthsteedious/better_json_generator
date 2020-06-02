use super::ParseContext;
use crate::ast::{Expression, json_array::JsonArrayExpression};

pub fn parse_array_expr(ctx: &mut ParseContext) -> impl Expression {
    JsonArrayExpression::new()
}