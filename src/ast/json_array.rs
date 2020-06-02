use super::{Expression, visitor::ExpressionVisitor};

#[derive(Debug)]
pub struct JsonArrayExpression{}

impl JsonArrayExpression {
    pub fn new() -> JsonArrayExpression {
        JsonArrayExpression{}
    }
}

impl Expression for JsonArrayExpression {
    fn accept(&mut self, visitor: &mut dyn ExpressionVisitor) {
        unimplemented!()
    }
}