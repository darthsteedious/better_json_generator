use super::{Expression, visitor::ExpressionVisitor};

#[derive(Debug)]
pub struct CommaExpression;

impl CommaExpression {
    pub fn new() -> CommaExpression {
        CommaExpression{}
    }
}

impl Expression for CommaExpression {
    fn accept(&mut self, visitor: &mut dyn ExpressionVisitor) {
        visitor.visit_comma_expression(self);
    }
}

