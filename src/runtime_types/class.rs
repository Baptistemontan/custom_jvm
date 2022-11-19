use std::sync::Arc;

use super::Method;

#[derive(Debug, Clone)]
pub struct Class {
    name: String,
    super_class: Option<Arc<Self>>,
    methods: Vec<Method>,
}

impl Class {
    pub fn get_superclass(&self) -> Option<&Arc<Self>> {
        self.super_class.as_ref()
    }

    pub fn is_subclass(self: &Arc<Self>, super_class: &Arc<Self>) -> bool {
        let mut current = Some(self);
        while let Some(class) = current {
            if Arc::ptr_eq(&class, super_class) {
                return true;
            } else {
                current = class.get_superclass();
            }
        }
        false
    }
}
