pub mod comma;
pub mod json_array;
pub mod json_object;
pub mod name;
pub mod property_assignment;
pub mod value;
pub mod visitor;
pub mod whitespace;
use visitor::ExpressionVisitor;

pub trait Expression: std::fmt::Debug {
    fn accept(&mut self, visitor: &mut dyn ExpressionVisitor);
}

#[derive(Debug)]
pub enum JsonValue {
    Number(i64),
    String(String),
    Boolean(bool),
    // Object(JsonObjectExpression),
    // Array(JsonArrayExpression)
}

#[cfg(test)]
mod tests {
    use super::*;
    use super::visitor::{JsonExpressionVisitor, ExpressionVisitor};
    use crate::parsing::ParseResult;
    use crate::ast::name::NameExpression;
    use crate::ast::value::ValueExpression;

    macro_rules! expr_theory {
        ($name:ident, $e:expr, $expected:literal) => {
            #[test]
            fn $name() {
                let mut v = JsonExpressionVisitor::new();
                let mut expr = $e;

                expr.accept(&mut v);
                assert_eq!($expected, v.get_json());
            }
        }
    }

    fn new_name_expr(name: &str) -> NameExpression {
        NameExpression::new(String::from(name))
    }

    fn new_value_expr(value: JsonValue) -> ValueExpression {
        ValueExpression::new(value)
    }

    fn new_prop_assignment(name: &str, value: JsonValue) -> property_assignment::PropertyAssignmentExpression {
        property_assignment::PropertyAssignmentExpression::new(
            Box::new(new_name_expr(name)),
            Box::new(new_value_expr(value)))
    }

    expr_theory!(number_assignment, new_prop_assignment("foo", JsonValue::Number(1)),
        "\"foo\": 1");
    expr_theory!(string_assignment, new_prop_assignment("foo", JsonValue::String(String::from("bar"))),
        "\"foo\": \"bar\"");
    expr_theory!(bool_assignment, new_prop_assignment("foo", JsonValue::Boolean(true)),
        "\"foo\": true");

    expr_theory!(comma_expression, comma::CommaExpression::new(), ",");
    expr_theory!(newline_expression, whitespace::WhitespaceExpression::new('\n'), "\n");
    expr_theory!(tab_expression, whitespace::WhitespaceExpression::new('\t'), "\t");
    expr_theory!(whitespace_expression, whitespace::WhitespaceExpression::new(' '), " ");

    macro_rules! obj_expr_theory {
        ($name:ident, $expected:literal, $e:expr) => {
            #[test]
            fn $name() {
                let mut v = JsonExpressionVisitor::new();
                let mut obj_expr = json_object::JsonObjectExpression::new();

                let nl: Box<dyn Expression> = Box::new(whitespace::WhitespaceExpression::new('\n'));
                let tab: Box<dyn Expression> = Box::new(whitespace::WhitespaceExpression::new('\t'));
                let assignment: Box<dyn Expression> = Box::new($e);
                let nl2: Box<dyn Expression> = Box::new(whitespace::WhitespaceExpression::new('\n'));

                obj_expr.add_expr(nl);
                obj_expr.add_expr(tab);
                obj_expr.add_expr(assignment);
                obj_expr.add_expr(nl2);

                obj_expr.accept(&mut v);

                assert_eq!($expected, v.get_json());
            }
        }
    }

    obj_expr_theory!(single_prop_string_object_expression, "{\n\t\"foo\": \"bar\"\n}",
        new_prop_assignment("foo", JsonValue::String(String::from("bar"))));

    obj_expr_theory!(single_prop_number_object_expression, "{\n\t\"foo\": 500\n}",
        new_prop_assignment("foo", JsonValue::Number(500)));

    obj_expr_theory!(single_prop_bool_object_expression, "{\n\t\"foo\": true\n}",
        new_prop_assignment("foo", JsonValue::Boolean(true)));
}