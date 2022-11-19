use std::sync::Arc;

use super::{Array, Reference};

#[derive(Debug, Clone)]
pub enum Object {
    Reference(Option<Reference>),
    Int(i32),
    Float(f32),
    Long(i64),
    Double(f64),
    Array(Option<Array>),
    Padding,
}

impl Object {
    pub fn is_wide(&self) -> bool {
        match self {
            Object::Double(_) | Object::Long(_) => true,
            _ => false,
        }
    }
}
