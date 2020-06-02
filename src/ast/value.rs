use super::{Expression, JsonValue, visitor::ExpressionVisitor};

#[derive(Debug)]
pub struct ValueExpression {
    pub value: JsonValue
}

impl ValueExpression {
    pub fn new(value: JsonValue) -> ValueExpression {
        ValueExpression {value}
    }
}

impl Expression for ValueExpression {
    fn accept(&mut self, visitor: &mut dyn ExpressionVisitor) {
        visitor.visit_value(self)
    }
}