use std::{
    ops::Deref,
    sync::{Arc, Mutex},
};

use crate::{parser::classfile::opcode::ArrayType, rethrow_exception};

use super::{Class, InternalError, Reference, ExecResult, Exception, Object, ResultValue};

#[derive(Debug, Clone)]
pub enum Array {
    Boolean(Arc<Mutex<Box<[bool]>>>),
    Char(Arc<Mutex<Box<[u8]>>>),
    Float(Arc<Mutex<Box<[f32]>>>),
    Double(Arc<Mutex<Box<[f64]>>>),
    Byte(Arc<Mutex<Box<[u8]>>>),
    Short(Arc<Mutex<Box<[i16]>>>),
    Int(Arc<Mutex<Box<[i32]>>>),
    Long(Arc<Mutex<Box<[i64]>>>),
    Reference(Arc<ReferenceArray>),
}

impl PartialEq for Array {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (Array::Boolean(array_1), Array::Boolean(array_2)) => Arc::ptr_eq(array_1, array_2),
            (Array::Char(array_1), Array::Char(array_2)) => Arc::ptr_eq(array_1, array_2),
            (Array::Float(array_1), Array::Float(array_2)) => Arc::ptr_eq(array_1, array_2),
            (Array::Double(array_1), Array::Double(array_2)) => Arc::ptr_eq(array_1, array_2),
            (Array::Byte(array_1), Array::Byte(array_2)) => Arc::ptr_eq(array_1, array_2),
            (Array::Short(array_1), Array::Short(array_2)) => Arc::ptr_eq(array_1, array_2),
            (Array::Int(array_1), Array::Int(array_2)) => Arc::ptr_eq(array_1, array_2),
            (Array::Long(array_1), Array::Long(array_2)) => Arc::ptr_eq(array_1, array_2),
            (Array::Reference(array_1), Array::Reference(array_2)) => Arc::ptr_eq(array_1, array_2),
            _ => false
        }
    }
}

impl Eq for Array {}

#[derive(Debug)]
pub struct ReferenceArray {
    array: Mutex<Box<[Option<Reference>]>>,
    class: Arc<Class>,
}

impl Deref for ReferenceArray {
    type Target = Mutex<Box<[Option<Reference>]>>;

    fn deref(&self) -> &Self::Target {
        &self.array
    }
}

impl ReferenceArray {
    pub fn get_class(&self) -> &Arc<Class> {
        &self.class
    }

    pub fn can_accept(&self, reference: &Reference) -> bool {
        reference.is_subclass(&self.class)
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

fn get_index_raw<T: Clone>(
    array: &Mutex<Box<[T]>>,
    index: usize,
) -> Result<Option<T>, InternalError> {
    let array = array.lock()?;
    Ok(array.get(index).cloned())
}

fn store_index_raw<T>(
    array: &Mutex<Box<[T]>>,
    index: usize,
    value: T
)-> Result<Result<(), ()>, InternalError> {
    let mut array = array.lock()?;
    if let Some(place_to_store) = array.get_mut(index) {
        *place_to_store = value;
        Ok(Ok(()))
    } else {
        Ok(Err(()))
    }
}

fn store_index<T>(array: &Mutex<Box<[T]>>, index: i32, value: T) -> ExecResult {
    if index >= 0 {
        if store_index_raw(array, index as usize, value)?.is_ok() {
            return Ok(Ok(ResultValue::None));
        }
    }
    // TODO: Throw ArrayIndexOutOfBoundsException
    todo!()
}

fn get_index<T: Clone>(array: &Mutex<Box<[T]>>, index: i32) -> Result<Result<T, Exception>, InternalError> {
    if index >= 0 {
        if let Some(elem) = get_index_raw(array, index as usize)? {
            return Ok(Ok(elem));
        }
    }
    // TODO: Throw ArrayIndexOutOfBoundsException
    todo!()
}

impl Array {
    pub fn new_reference(class: Arc<Class>, size: usize) -> Self {
        Array::Reference(Arc::new(ReferenceArray {
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
            Array::Reference(array) => get_size(&array),
        }
    }

    pub fn get_index(&self, index: i32) -> ExecResult {
        let value = match self {
            Array::Boolean(array) => {
                let value = rethrow_exception!(get_index(array, index)?);
                Object::Int(value as i32)   
            },
            Array::Char(array) => {
                let value = rethrow_exception!(get_index(array, index)?);
                Object::Int(value as i32)
            },
            Array::Float(array) => {
                let value = rethrow_exception!(get_index(array, index)?);
                Object::Float(value)
            },
            Array::Double(array) => {
                let value = rethrow_exception!(get_index(array, index)?);
                Object::Double(value)
            },
            Array::Byte(array) => {
                let value = rethrow_exception!(get_index(array, index)?);
                Object::Int(value as i32)
            },
            Array::Short(array) => {
                let value = rethrow_exception!(get_index(array, index)?);
                Object::Int(value as i32)
            },
            Array::Int(array) => {
                let value = rethrow_exception!(get_index(array, index)?);
                Object::Int(value)
            },
            Array::Long(array) => {
                let value = rethrow_exception!(get_index(array, index)?);
                Object::Long(value)
            },
            Array::Reference(array) => {
                let value = rethrow_exception!(get_index(array, index)?);
                Object::Reference(value)
            },
        };
        Ok(Ok(ResultValue::Object(value)))
    }

    pub fn store_index(&self, index: i32, value: Object) -> ExecResult {
        match (self, value) {
            (Array::Boolean(array), Object::Int(value)) => {
                let value = if value % 2 == 0 { false } else { true };
                store_index(array, index, value)
            },
            (Array::Char(array), Object::Int(value)) => {
                store_index(array, index, value as u8)
            },
            (Array::Float(array), Object::Float(value)) => {
                store_index(array, index, value)
            },
            (Array::Double(array), Object::Double(value)) => {
                store_index(array, index, value)
            },
            (Array::Byte(array), Object::Int(value)) => {
                store_index(array, index, value as u8)
            },
            (Array::Short(array), Object::Int(value)) => {
                store_index(array, index, value as i16)
            },
            (Array::Int(array), Object::Int(value)) => {
                store_index(array, index, value)
            },
            (Array::Long(array), Object::Long(value)) => {
                store_index(array, index, value)
            },
            (Array::Reference(array), Object::Reference(reference)) => {
                if let Some(reference) = &reference {
                    if !array.can_accept(reference) {
                        // TODO: throw ArrayStoreException
                        todo!()
                    }
                }
                store_index(array, index, reference)
            },
            _ => Err(InternalError::WrongType)
        }
    }
}
