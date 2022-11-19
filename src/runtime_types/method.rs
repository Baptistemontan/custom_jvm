use std::sync;

use super::{Class, Code};

#[derive(Debug, Clone)]
pub struct Method {
    name: String,
    descriptor: String,
    class: sync::Weak<Class>,
    code: Code,
}
