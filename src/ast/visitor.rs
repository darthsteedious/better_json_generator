use super::{
    JsonValue,
    comma::CommaExpression,
    json_object::JsonObjectExpression,
    property_assignment::PropertyAssignmentExpression,
    whitespace::WhitespaceExpression};


pub trait ExpressionVisitor {
    fn visit_object(&mut self, expr: &mut JsonObjectExpression);
    fn visit_property_assignment(&mut self, expr: &PropertyAssignmentExpression);
    fn visit_comma_expression(&mut self, expr: &CommaExpression);
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
    fn visit_object(&mut self, expr: &mut JsonObjectExpression) {
        self.json = format!("{}{{", self.json);

        for pae in expr.expressions.iter_mut() {
            pae.accept(self);
        }

        self.json = format!("{}}}", self.json);
    }

    fn visit_property_assignment(&mut self, expr: &PropertyAssignmentExpression) {
        let value = match &expr.value {
            JsonValue::Number(i) => format!("{}", i).to_string(),
            JsonValue::Boolean(b) => format!("{}", b).to_string(),
            JsonValue::String(s) => format!("\"{}\"", s).to_string()
        };

        self.json = format!("{}\"{}\": {}", self.json, &expr.name, &value);
    }

    fn visit_comma_expression(&mut self, _: &CommaExpression) {
        self.json = format!("{},", self.json);
    }

    fn visit_whitespace_expression(&mut self, e: &WhitespaceExpression) {
        self.json = format!("{}{}", self.json, e.get_char());
    }

    fn get_json(&self) -> &str {
        &self.json
    }
}