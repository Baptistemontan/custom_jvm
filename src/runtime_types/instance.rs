use super::Class;
use std::sync::Arc;
// use std::sync::Mutex;
use std::ops::Deref;

#[derive(Debug, Clone)]
pub struct Instance(Arc<InstanceInner>);

#[derive(Debug)]
pub struct InstanceInner {
    class: Arc<Class>,
}

impl Deref for Instance {
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
