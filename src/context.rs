use crate::node::{Expression, Statement};

pub struct Context<'a> {
    pub is_function_identifier: bool,
    pub is_directive: bool,
    pub is_pattern: bool,
    pub statements: &'a mut Vec<Box<dyn Statement>>,
    pub expressions: Option<&'a mut Vec<Box<dyn Expression>>>,
}

impl<'a> Context<'a> {
    pub fn new(statements: &'a mut Vec<Box<dyn Statement>>) -> Self {
        Context {
            statements,
            expressions: None,
            is_function_identifier: false,
            is_directive: false,
            is_pattern: false,
        }
    }
}
