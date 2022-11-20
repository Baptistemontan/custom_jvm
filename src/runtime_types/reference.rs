use super::Class;
use std::sync::Arc;
// use std::sync::Mutex;
use std::ops::Deref;

#[derive(Debug, Clone)]
pub struct Reference(Arc<RefInner>);

#[derive(Debug)]
pub struct RefInner {
    class: Arc<Class>,
}

impl Deref for Reference {
    type Target = RefInner;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl RefInner {
    pub fn get_class(&self) -> &Arc<Class> {
        &self.class
    }
}

impl Reference {
    pub fn is_subclass(&self, super_class: &Arc<Class>) -> bool {
        self.get_class().is_subclass(super_class)
    }
}

impl PartialEq for Reference {
    fn eq(&self, rhs: &Self) -> bool {
        Arc::ptr_eq(&self.0, &rhs.0)
    }
}

impl Eq for Reference {}
