use std::sync;

use super::{Class, Code, MethodCallResult, Stack};

#[derive(Debug, Clone)]
pub struct Method {
    name: String,
    descriptor: String,
    class: sync::Weak<Class>,
    code: Code,
}

impl Method {
    pub fn execute(&self, caller_stack: &mut Stack) -> MethodCallResult {
        self.code.execute(caller_stack)
    }
}
