use super::{Expression, visitor::ExpressionVisitor};

#[derive(Debug)]
pub struct NameExpression {
    pub name: String
}

impl NameExpression {
    pub fn new(name: String) -> NameExpression {
        NameExpression { name }
    }
}

impl Expression for NameExpression {
    fn accept(&mut self, visitor: &mut dyn ExpressionVisitor) {
        visitor.visit_name(self)
    }
}