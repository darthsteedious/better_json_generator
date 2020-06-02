use super::{
    JsonValue,
    comma::CommaExpression,
    json_object::JsonObjectExpression,
    name::NameExpression,
    property_assignment::PropertyAssignmentExpression,
    whitespace::WhitespaceExpression};
use crate::ast::json_array::JsonArrayExpression;
use crate::ast::value::ValueExpression;


pub trait ExpressionVisitor {
    fn visit_array(&mut self, expr: &mut JsonArrayExpression);
    fn visit_object(&mut self, expr: &mut JsonObjectExpression);
    fn visit_name(&mut self, expr: &mut NameExpression);
    fn visit_property_assignment(&mut self, expr: &mut PropertyAssignmentExpression);
    fn visit_comma_expression(&mut self, expr: &CommaExpression);
    fn visit_value(&mut self, expr: &mut ValueExpression);
    fn visit_whitespace_expression(&mut self, expr: &WhitespaceExpression);

    fn get_json(&self) -> &str;
}

pub struct JsonExpressionVisitor {
    pub json: String
}

impl JsonExpressionVisitor {
    pub fn new() -> impl ExpressionVisitor {
        JsonExpressionVisitor { json: String::new() }
    }
}

impl ExpressionVisitor for JsonExpressionVisitor {
    fn visit_array(&mut self, expr: &mut JsonArrayExpression) {
        self.json = format!("{}[", self.json);

        // TODO: Implement

        self.json = format!("{}]", self.json);
    }

    fn visit_object(&mut self, expr: &mut JsonObjectExpression) {
        self.json = format!("{}{{", self.json);

        for pae in expr.expressions.iter_mut() {
            pae.accept(self);
        }

        self.json = format!("{}}}", self.json);
    }

    fn visit_name(&mut self, expr: &mut NameExpression) {
        self.json = format!("{}\"{}\"", self.json, expr.name);
    }

    fn visit_property_assignment(&mut self, expr: &mut PropertyAssignmentExpression) {
        expr.name.accept(self);
        self.json = format!("{}: ", self.json);
        expr.value.accept(self);
    }

    fn visit_comma_expression(&mut self, _: &CommaExpression) {
        self.json = format!("{},", self.json);
    }

    fn visit_value(&mut self, expr: &mut ValueExpression) {
        self.json = match &expr.value {
            JsonValue::String(s) => format!("{}\"{}\"", self.json, *s),
            JsonValue::Number(d) => format!("{}{}", self.json, *d),
            JsonValue::Boolean(b) => format!("{}{}", self.json, *b)
        }
    }

    fn visit_whitespace_expression(&mut self, e: &WhitespaceExpression) {
        self.json = format!("{}{}", self.json, e.get_char());
    }

    fn get_json(&self) -> &str {
        &self.json
    }
}