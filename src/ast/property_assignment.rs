use super::{Expression, JsonValue, name::NameExpression, visitor::ExpressionVisitor};

#[derive(Debug)]
pub struct PropertyAssignmentExpression {
    pub name: Box<dyn Expression>,
    pub value: Box<dyn Expression>,
}

impl PropertyAssignmentExpression {
    pub fn new(name: Box<dyn Expression>, value: Box<dyn Expression>) -> PropertyAssignmentExpression {
        PropertyAssignmentExpression { name, value }
    }
}

impl Expression for PropertyAssignmentExpression {
    fn accept(&mut self, visitor: &mut dyn ExpressionVisitor) {
        visitor.visit_property_assignment(self);
    }
}