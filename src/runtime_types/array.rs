use std::{
    ops::Deref,
    sync::{Arc, Mutex},
};

use crate::parser::classfile::opcode::ArrayType;

use super::{Class, Instance, InternalError};

#[derive(Debug, Clone)]
pub enum Array {
    Boolean(Arc<Mutex<Box<[bool]>>>),
    Char(Arc<Mutex<Box<[char]>>>),
    Float(Arc<Mutex<Box<[f32]>>>),
    Double(Arc<Mutex<Box<[f64]>>>),
    Byte(Arc<Mutex<Box<[u8]>>>),
    Short(Arc<Mutex<Box<[i16]>>>),
    Int(Arc<Mutex<Box<[i32]>>>),
    Long(Arc<Mutex<Box<[i64]>>>),
    Instance(Arc<InstanceArray>),
}

#[derive(Debug)]
pub struct InstanceArray {
    array: Mutex<Box<[Option<Instance>]>>,
    class: Arc<Class>,
}

impl Deref for InstanceArray {
    type Target = Mutex<Box<[Option<Instance>]>>;

    fn deref(&self) -> &Self::Target {
        &self.array
    }
}

impl InstanceArray {
    pub fn get_class(&self) -> &Arc<Class> {
        &self.class
    }

    pub fn size(&self) -> Result<i32, InternalError> {
        get_size(&self.array)
    }
}

fn new_boxed_array<T: Default>(size: usize) -> Box<[T]> {
    let mut v = Vec::with_capacity(size);
    let default_iter = std::iter::repeat_with(|| T::default()).take(size);
    v.extend(default_iter);
    v.into_boxed_slice()
}

fn get_size<T>(array: &Mutex<Box<[T]>>) -> Result<i32, InternalError> {
    let array = array.lock()?;
    let size = array.len() as i32;
    Ok(size)
}

impl Array {
    pub fn new_instance(class: Arc<Class>, size: usize) -> Self {
        Array::Instance(Arc::new(InstanceArray {
            array: Mutex::new(new_boxed_array(size)),
            class,
        }))
    }

    pub fn new_numerical(array_type: ArrayType, size: usize) -> Self {
        match array_type {
            ArrayType::Boolean => Array::Boolean(Arc::new(Mutex::new(new_boxed_array(size)))),
            ArrayType::Char => Array::Char(Arc::new(Mutex::new(new_boxed_array(size)))),
            ArrayType::Float => Array::Float(Arc::new(Mutex::new(new_boxed_array(size)))),
            ArrayType::Double => Array::Double(Arc::new(Mutex::new(new_boxed_array(size)))),
            ArrayType::Byte => Array::Byte(Arc::new(Mutex::new(new_boxed_array(size)))),
            ArrayType::Short => Array::Short(Arc::new(Mutex::new(new_boxed_array(size)))),
            ArrayType::Int => Array::Int(Arc::new(Mutex::new(new_boxed_array(size)))),
            ArrayType::Long => Array::Long(Arc::new(Mutex::new(new_boxed_array(size)))),
        }
    }

    pub fn size(&self) -> Result<i32, InternalError> {
        match self {
            Array::Boolean(array) => get_size(&array),
            Array::Char(array) => get_size(&array),
            Array::Float(array) => get_size(&array),
            Array::Double(array) => get_size(&array),
            Array::Byte(array) => get_size(&array),
            Array::Short(array) => get_size(&array),
            Array::Int(array) => get_size(&array),
            Array::Long(array) => get_size(&array),
            Array::Instance(array) => get_size(&array),
        }
    }

    pub fn get_index<T: Clone>(
        array: &Mutex<Box<[T]>>,
        index: usize,
    ) -> Result<Option<T>, InternalError> {
        let array = array.lock()?;
        Ok(array.get(index).cloned())
    }
}
