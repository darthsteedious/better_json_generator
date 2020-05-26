use super::{Expression, visitor::ExpressionVisitor};

#[derive(Debug)]
pub struct JsonObjectExpression {
    pub expressions: Vec<Box<dyn Expression>>,
}

impl JsonObjectExpression{
    pub fn new() -> JsonObjectExpression {
        JsonObjectExpression{expressions: Vec::new() }
    }
    pub fn add_expr(&mut self, expr: Box<dyn Expression>) {
        self.expressions.push(expr);
    }
}

impl Expression for JsonObjectExpression {
    fn accept(&mut self, visitor: &mut dyn ExpressionVisitor) {
        visitor.visit_object(self);
    }
}

impl Expression for &mut JsonObjectExpression {
    fn accept(&mut self, visitor: &mut dyn ExpressionVisitor) {
        visitor.visit_object(self);
    }
}