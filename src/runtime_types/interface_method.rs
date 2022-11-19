use std::sync::Arc;

use super::Class;

#[derive(Debug, Clone)]
pub struct InterfaceMethod {
    signature: String,
    interface: Arc<Class>,
}
