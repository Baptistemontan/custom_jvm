use std::{ops::RangeInclusive, sync::Arc};

use super::{Class, Instance, InternalError, Object, OpCode};

pub enum ResultValue {
    None,
    Object(Object),
    Return,
    ReturnObject(Object),
}

pub type Exception = Instance;
pub type OpResult = Result<ResultValue, Exception>;
pub type ExecResult = Result<OpResult, InternalError>;

#[derive(Debug, Clone)]
pub struct ExceptionTableInfo {
    code_range: RangeInclusive<usize>,
    handler_pc: usize,
    catch_type: Arc<Class>,
}

impl ExceptionTableInfo {
    fn does_handle(&self, current_pc: usize, exception_class: &Arc<Class>) -> Option<usize> {
        if self.code_range.contains(&current_pc) && exception_class.is_subclass(&self.catch_type) {
            Some(self.handler_pc)
        } else {
            None
        }
    }
}

#[derive(Debug, Clone)]
pub struct ExceptionTable {
    infos: Vec<ExceptionTableInfo>,
}

impl ExceptionTable {
    fn get_jump(&self, current_pc: usize, exception_class: &Arc<Class>) -> Option<usize> {
        self.infos
            .iter()
            .find_map(|info| info.does_handle(current_pc, exception_class))
    }
}

#[derive(Debug, Clone)]
pub struct Code {
    max_stack: usize,
    max_locals: usize,
    opcodes: Vec<OpCode>,
    exception_table: ExceptionTable,
}

impl Code {
    pub fn execute(&self) -> Result<Result<Option<Object>, Exception>, InternalError> {
        let mut programm_counter = 0;
        let mut locals = Locals::new(self.max_locals);
        let mut stack = Stack::new(self.max_stack);
        loop {
            let Some(opcode) = self.opcodes.get(programm_counter) else {
                return Err(InternalError::InvalidProgrammCounter);
            };
            let result = opcode.execute(&mut locals, &mut stack)?;
            match result {
                Ok(ResultValue::None) => {
                    programm_counter += 1;
                }
                Ok(ResultValue::Object(value)) => {
                    stack.push(value);
                    programm_counter += 1;
                }
                Ok(ResultValue::Return) => {
                    return Ok(Ok(None));
                }
                Ok(ResultValue::ReturnObject(return_value)) => {
                    return Ok(Ok(Some(return_value)));
                }
                Err(exception) => {
                    let exception_class = exception.get_class();
                    let Some(handle_pc) = self.exception_table.get_jump(programm_counter, exception_class) else {
                        return Ok(Err(exception));
                    };
                    programm_counter = handle_pc;
                    stack.clear();
                }
            }
        }
    }
}

pub struct Stack {
    stack: Vec<Object>,
}

impl Stack {
    pub fn new(max_size: usize) -> Self {
        Stack {
            stack: Vec::with_capacity(max_size),
        }
    }

    pub fn pop(&mut self) -> Result<Object, InternalError> {
        self.stack.pop().ok_or(InternalError::EmptyStack)
    }

    pub fn push(&mut self, value: Object) {
        self.stack.push(value);
    }

    pub fn clear(&mut self) {
        self.stack.clear();
    }
}

pub struct Locals {
    locals: Vec<Option<Object>>,
}

impl Locals {
    pub fn new(size: usize) -> Self {
        let mut locals = Vec::with_capacity(size);
        let iter = std::iter::repeat_with(|| None).take(size);
        locals.extend(iter);
        Locals { locals }
    }

    pub fn load(&self, index: usize) -> Result<Option<Object>, InternalError> {
        self.locals
            .get(index)
            .cloned()
            .ok_or(InternalError::LocalsOutOfBounds)
    }

    pub fn load_non_empty(&self, index: usize) -> Result<Object, InternalError> {
        self.load(index)?.ok_or(InternalError::EmptyLocals)
    }

    pub fn load_non_empty_wide(&self, index: usize) -> Result<Object, InternalError> {
        let result = self.load_non_empty(index)?;
        let padding = self.load_non_empty(index + 1)?;
        match padding {
            Object::Padding => Ok(result),
            _ => Err(InternalError::InvalidWideLoad),
        }
    }

    fn get_index_mut(&mut self, index: usize) -> Result<&mut Option<Object>, InternalError> {
        self.locals
            .get_mut(index)
            .ok_or(InternalError::LocalsOutOfBounds)
    }

    pub fn store(&mut self, index: usize, value: Object) -> Result<(), InternalError> {
        let place_to_store = self.get_index_mut(index)?;
        *place_to_store = Some(value);
        Ok(())
    }

    pub fn store_wide(&mut self, index: usize, value: Object) -> Result<(), InternalError> {
        self.store(index, value)?;
        self.store(index + 1, Object::Padding)
    }
}
