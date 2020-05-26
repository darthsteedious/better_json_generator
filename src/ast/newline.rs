use super::{Expression, visitor::ExpressionVisitor};

pub struct NewlineExpression;

impl NewlineExpression {
    pub fn new() -> NewlineExpression {
        NewlineExpression{}
    }
}

impl Expression for NewlineExpression {
    fn accept(&mut self, visitor: &mut dyn ExpressionVisitor) {
        visitor.visit_newline_expression(self);
    }
}