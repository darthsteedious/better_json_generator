use super::{Expression, JsonValue, visitor::ExpressionVisitor};

#[derive(Debug)]
pub struct PropertyAssignmentExpression {
    pub name: String,
    pub value: JsonValue,
}

impl PropertyAssignmentExpression {
    pub fn new(name: String, value: JsonValue) -> PropertyAssignmentExpression {
        PropertyAssignmentExpression { name, value }
    }
}

impl Expression for PropertyAssignmentExpression {
    fn accept(&mut self, visitor: &mut dyn ExpressionVisitor) {
        visitor.visit_property_assignment(self);
    }
}