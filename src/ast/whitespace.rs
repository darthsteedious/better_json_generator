use super::{Expression, visitor::ExpressionVisitor};

#[derive(Debug)]
pub struct WhitespaceExpression {
    c: char
}

impl WhitespaceExpression {
    pub fn new(c: char) -> WhitespaceExpression {
        WhitespaceExpression{c}
    }

    pub fn get_char(&self) -> char {
        self.c
    }
}

impl Expression for WhitespaceExpression {
    fn accept(&mut self, visitor: &mut dyn ExpressionVisitor) {
        visitor.visit_whitespace_expression(self);
    }
}