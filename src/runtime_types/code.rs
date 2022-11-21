use std::{ops::RangeInclusive, sync::Arc};

use super::{Class, InternalError, Object, OpCode, Reference};

pub enum ResultValue {
    None,
    Object(Object),
    Return,
    ReturnObject(Object),
    Jump(usize),
}

pub type Exception = Reference;
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
    infos: Option<Vec<ExceptionTableInfo>>,
}

impl ExceptionTable {
    pub fn new(infos: Option<Vec<ExceptionTableInfo>>) -> Self {
        ExceptionTable { infos }
    }

    fn get_jump(&self, current_pc: usize, exception_class: &Arc<Class>) -> Option<usize> {
        self.infos
            .as_ref()?
            .iter()
            .find_map(|info| info.does_handle(current_pc, exception_class))
    }
}

#[derive(Debug, Clone)]
pub struct Code {
    max_stack: usize,
    max_locals: usize,
    opcodes: Vec<OpCode>,
    args_count: usize,
    exception_table: ExceptionTable,
}

pub type MethodCallResult = Result<Result<Option<Object>, Exception>, InternalError>;

impl Code {
    pub fn new(
        max_stack: usize,
        max_locals: usize,
        opcodes: Vec<OpCode>,
        args_count: usize,
        exception_table: ExceptionTable,
    ) -> Self {
        Code {
            max_stack,
            max_locals,
            opcodes,
            args_count,
            exception_table,
        }
    }

    fn create_locals(&self, stack: &mut Stack) -> Result<Locals, InternalError> {
        Locals::from_stack(self.max_locals, self.args_count, stack)
    }

    pub fn execute(&self, caller_stack: &mut Stack) -> MethodCallResult {
        let mut locals = self.create_locals(caller_stack)?;
        let mut stack = Stack::new(self.max_stack);
        let mut programm_counter = 0;
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
                Ok(ResultValue::Jump(jump)) => {
                    programm_counter = jump;
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

    fn pop_single(&mut self) -> Result<Object, InternalError> {
        self.stack.pop().ok_or(InternalError::EmptyStack)
    }

    fn peek(&self) -> Result<&Object, InternalError> {
        self.stack.last().ok_or(InternalError::EmptyStack)
    }

    pub fn pop(&mut self) -> Result<Object, InternalError> {
        let value = self.pop_single()?;
        if value.is_wide() {
            let padding = self.pop_single()?;
            match padding {
                Object::Padding => Ok(value),
                _ => Err(InternalError::InvalidWideLoad),
            }
        } else {
            Ok(value)
        }
    }

    /// Duplicate the value on top of the stack and put it at the top
    ///
    /// Only dup the top element, wheter it is wide or not
    /// for wide elements see dup2
    pub fn dup(&mut self) -> Result<(), InternalError> {
        let top_value = self.peek()?.clone();
        self.push(top_value);
        Ok(())
    }

    /// Duplicate the value at the top of the stack and put it at the third place
    ///
    /// Only dup the top element, wether it is wide or note
    /// put it in third place wether are not the second element is wide are not
    /// for wide elements see dup2_x1
    pub fn dup_x1(&mut self) -> Result<(), InternalError> {
        // My first approach was to dup the top value, then swap the 2 value underneath
        // but [T]::swap panic if one of the index is OOB and there is no try variant
        // so pop the 2 top values, then clone insert the first, insert second, then reinsert first
        let top = self.pop_single()?;
        let second = self.pop_single()?;
        self.stack.extend([top.clone(), second, top]);
        Ok(())
    }

    /// Duplicate the value at the top of the stack and put it at the fourth place
    ///
    /// This operation handle the case when the second element is wide.
    pub fn dup_x2(&mut self) -> Result<(), InternalError> {
        let top = self.pop_single()?;
        let second = self.pop()?; // need to pop to remove padding if second is wide
        let third = if second.is_wide() {
            Object::Padding
        } else {
            self.pop_single()?
        };
        self.stack.extend([top.clone(), third, second, top]);
        Ok(())
    }

    /// Duplicate the top one or two operand stack values
    ///
    /// Duplicate the top element if it is wide, if not the case duplicate also the second
    pub fn dup2(&mut self) -> Result<(), InternalError> {
        let top = self.pop()?;
        if top.is_wide() {
            self.push(top.clone());
            self.push(top);
        } else {
            let second = self.pop_single()?;
            self.stack.push(second);
        }
        Ok(())
    }

    pub fn dup2_x1(&mut self) -> Result<(), InternalError> {
        let top = self.pop()?;
        let second = self.pop_single()?;
        if top.is_wide() {
            self.stack
                .extend([Object::Padding, top.clone(), second, Object::Padding, top]);
        } else {
            let third = self.pop_single()?;
            self.stack
                .extend([second.clone(), top.clone(), third, second, top]);
        }
        Ok(())
    }

    pub fn dup2_x2(&mut self) -> Result<(), InternalError> {
        let top = self.pop()?;
        let second = self.pop()?;
        match (top.is_wide(), second.is_wide()) {
            (true, true) => {
                self.stack.extend([
                    Object::Padding,
                    top.clone(),
                    Object::Padding,
                    second,
                    Object::Padding,
                    top,
                ]);
            }
            (true, false) => {
                let third = self.pop_single()?;
                self.stack.extend([
                    Object::Padding,
                    top.clone(),
                    third,
                    second,
                    Object::Padding,
                    top,
                ]);
            }
            (false, true) => {
                // dafuk
                // I don't really know what to do there
                return Err(InternalError::InvalidWideLoad);
            }
            (false, false) => {
                let third = self.pop()?;
                if third.is_wide() {
                    self.stack.extend([
                        second.clone(),
                        top.clone(),
                        Object::Padding,
                        third,
                        second,
                        top,
                    ]);
                } else {
                    let fourth = self.pop_single()?;
                    self.stack
                        .extend([second.clone(), top.clone(), fourth, third, second, top])
                }
            }
        }
        Ok(())
    }

    pub fn swap(&mut self) -> Result<(), InternalError> {
        let top = self.pop_single()?;
        let second = self.pop_single()?;
        self.stack.extend([top, second]);
        Ok(())
    }

    pub fn push(&mut self, value: Object) {
        let is_wide = value.is_wide();
        self.stack.push(value);
        if is_wide {
            self.stack.push(Object::Padding);
        }
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

    pub fn from_stack(
        max_size: usize,
        arg_count: usize,
        stack: &mut Stack,
    ) -> Result<Self, InternalError> {
        let mut locals = Self::new(max_size);
        for i in 0..arg_count {
            let value = stack.pop_single()?;
            let Some(place_to_store) = locals.locals.get_mut(i) else {
                return Err(InternalError::LocalsOutOfBounds);
            };
            *place_to_store = Some(value);
        }
        Ok(locals)
    }

    pub fn load(&self, index: usize) -> Result<Option<Object>, InternalError> {
        self.locals
            .get(index)
            .cloned()
            .ok_or(InternalError::LocalsOutOfBounds)
    }

    fn try_load_non_empty(&self, index: usize) -> Result<Object, InternalError> {
        self.load(index)?.ok_or(InternalError::EmptyLocals)
    }

    pub fn load_non_empty(&self, index: usize) -> Result<Object, InternalError> {
        let value = self.try_load_non_empty(index)?;
        if value.is_wide() {
            let padding = self.try_load_non_empty(index + 1)?;
            match padding {
                Object::Padding => Ok(value),
                _ => Err(InternalError::InvalidWideLoad),
            }
        } else {
            Ok(value)
        }
    }

    pub fn get_mut(&mut self, index: usize) -> Result<&mut Option<Object>, InternalError> {
        self.locals
            .get_mut(index)
            .ok_or(InternalError::LocalsOutOfBounds)
    }

    pub fn get_non_empty_mut(&mut self, index: usize) -> Result<&mut Object, InternalError> {
        let object = self.get_mut(index)?;
        object.as_mut().ok_or(InternalError::EmptyLocals)
    }

    fn try_store(&mut self, index: usize, value: Object) -> Result<(), InternalError> {
        let place_to_store = self.get_mut(index)?;
        *place_to_store = Some(value);
        Ok(())
    }

    pub fn store(&mut self, index: usize, value: Object) -> Result<(), InternalError> {
        let is_wide = value.is_wide();
        self.try_store(index, value)?;
        if is_wide {
            self.try_store(index + 1, Object::Padding)?;
        }
        Ok(())
    }
}
