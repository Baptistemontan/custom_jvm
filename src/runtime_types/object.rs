use std::sync::Arc;

use super::{Array, Instance};

#[derive(Debug, Clone)]
pub enum Object {
    Instance(Option<Instance>),
    Int(i32),
    Float(f32),
    Long(i64),
    Double(f64),
    Array(Option<Array>),
    Padding,
}
