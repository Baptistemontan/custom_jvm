use super::Class;
use std::sync::Arc;
// use std::sync::Mutex;
use std::ops::Deref;

#[derive(Debug, Clone)]
pub struct Reference(Arc<InstanceInner>);

#[derive(Debug)]
pub struct InstanceInner {
    class: Arc<Class>,
}

impl Deref for Reference {
    type Target = InstanceInner;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl InstanceInner {
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
