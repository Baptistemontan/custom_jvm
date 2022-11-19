use crate::parser::classfile::opcode::{ArrayType, LookupSwitch, TableSwitch};
use std::sync::Arc;

use super::{Array, Class, Field, Instance, InterfaceMethod, InternalError, Method, Object, Exception, OpResult, ResultValue, Stack, ExecResult, Locals};

#[derive(Debug, Clone, Copy)]
pub enum ConstantNumerical {
    Double(f64),
    Long(i64),
}

#[derive(Debug, Clone)]
#[allow(non_camel_case_types)]
pub enum OpCode {
    aaload,
    aastore,
    aconst_null,
    aload {
        local_index: usize,
    },
    aload_0,
    aload_1,
    aload_2,
    aload_3,
    anewarray {
        item_class: Arc<Class>,
    },
    areturn,
    arraylength,
    astore {
        local_index: usize,
    },
    astore_0,
    astore_1,
    astore_2,
    astore_3,
    athrow,
    baload,
    bastore,
    bipush(i32),
    caload,
    castore,
    checkcast {
        class: Arc<Class>,
    },
    d2f,
    d2i,
    d2l,
    dadd,
    daload,
    dastore,
    dcmpg,
    dcmpl,
    dconst_0,
    dconst_1,
    ddiv,
    dload {
        local_index: usize,
    },
    dload_0,
    dload_1,
    dload_2,
    dload_3,
    dmul,
    dneg,
    drem,
    dreturn,
    dstore {
        local_index: usize,
    },
    dstore_0,
    dstore_1,
    dstore_2,
    dstore_3,
    dsub,
    dup,
    dup_x1,
    dup_x2,
    dup2,
    dup2_x1,
    dup2_x2,
    f2d,
    f2i,
    f2l,
    fadd,
    faload,
    fastore,
    fcmpg,
    fcmpl,
    fconst_0,
    fconst_1,
    fconst_2,
    fdiv,
    fload {
        local_index: usize,
    },
    fload_0,
    fload_1,
    fload_2,
    fload_3,
    fmul,
    fneg,
    frem,
    freturn,
    fstore {
        local_index: usize,
    },
    fstore_0,
    fstore_1,
    fstore_2,
    fstore_3,
    fsub,
    getfield {
        field_name: String,
    },
    getstatic {
        field: Arc<Field>,
    },
    goto(usize),
    goto_w(usize),
    i2b,
    i2c,
    i2d,
    i2f,
    i2l,
    i2s,
    iadd,
    iaload,
    iand,
    iastore,
    iconst_m1,
    iconst_0,
    iconst_1,
    iconst_2,
    iconst_3,
    iconst_4,
    iconst_5,
    idiv,
    if_acmpeq(usize),
    if_acmpne(usize),
    if_icmpeq(usize),
    if_icmpne(usize),
    if_icmplt(usize),
    if_icmpge(usize),
    if_icmpgt(usize),
    if_icmple(usize),
    ifeq(usize),
    ifne(usize),
    iflt(usize),
    ifge(usize),
    ifgt(usize),
    ifle(usize),
    ifnonnull(usize),
    ifnull(usize),
    iinc {
        local_index: usize,
        delta: i32,
    },
    iload {
        local_index: usize,
    },
    iload_0,
    iload_1,
    iload_2,
    iload_3,
    imul,
    ineg,
    instanceof {
        class: Arc<Class>,
    },
    // TODO
    invokedynamic(usize),
    invokeinterface {
        interface_method: Arc<InterfaceMethod>,
        count: usize,
    },
    invokespecial {
        method_signature: Arc<String>,
    },
    invokestatic {
        method: Arc<Method>,
    },
    invokevirtual {
        method_signature: Arc<String>,
    },
    ior,
    irem,
    ireturn,
    ishl,
    ishr,
    istore {
        local_index: usize,
    },
    istore_0,
    istore_1,
    istore_2,
    istore_3,
    isub,
    iushr,
    ixor,
    jsr(usize),
    jsr_w(usize),
    l2d,
    l2f,
    l2i,
    ladd,
    laload,
    land,
    lastore,
    lcmp,
    lconst_0,
    lconst_1,
    // TODO:
    ldc(usize),
    // TODO:
    ldc_w(usize),
    ldc2_w(ConstantNumerical),
    ldiv,
    lload {
        local_index: usize,
    },
    lload_0,
    lload_1,
    lload_2,
    lload_3,
    lmul,
    lneg,
    lookupswitch(LookupSwitch),
    lor,
    lrem,
    lreturn,
    lshl,
    lshr,
    lstore {
        local_index: usize,
    },
    lstore_0,
    lstore_1,
    lstore_2,
    lstore_3,
    lsub,
    lushr,
    lxor,
    monitorenter,
    monitorexit,
    multinewarray {
        item_class: Arc<Class>,
        dimensions: usize,
    },
    new {
        class: Arc<Class>,
    },
    newarray(ArrayType),
    nop,
    pop,
    pop2,
    putfield {
        field_name: String,
    },
    putstatic {
        field: Arc<Field>,
    },
    ret {
        local_index: usize,
    },
    retrn,
    saload,
    sastore,
    sipush(i32),
    swap,
    tableswitch(TableSwitch),
    // TODO
    // wide(Wide),
}

impl OpCode {
    pub fn execute(&self, locals: &mut Locals, stack: &mut Stack) -> ExecResult {
        use OpCode::*;
        match self {
            aaload => exec_aaload(stack),
            aastore => exec_aastore(stack),
            aconst_null => Ok(Ok(ResultValue::Object(Object::Instance(None)))), // yep that's a long wrapping for null
            aload { local_index } => exec_load_local(locals, *local_index, false),
            aload_0 => exec_load_local(locals, 0, false),
            aload_1 => exec_load_local(locals, 1, false),
            aload_2 => exec_load_local(locals, 2, false),
            aload_3 => exec_load_local(locals, 3, false),
            anewarray { item_class } => exec_anewarray(stack, item_class),
            areturn => exec_return_with_value(stack),
            arraylength => exec_arraylength(stack),
            astore { local_index } => exec_store_local(locals, stack, *local_index, false),
            astore_0 => exec_store_local(locals, stack, 0, false),
            astore_1 => exec_store_local(locals, stack, 1, false),
            astore_2 => exec_store_local(locals, stack, 2, false),
            astore_3 => exec_store_local(locals, stack, 3, false),
            athrow => exec_athrow(stack),
            baload => exec_baload(stack),
            bastore => exec_bastore(stack),
            bipush(value) => Ok(Ok(ResultValue::Object(Object::Int(*value)))),
            caload => todo!(),
            castore => todo!(),
            checkcast { class } => todo!(),
            d2f => todo!(),
            d2i => todo!(),
            d2l => todo!(),
            dadd => todo!(),
            daload => todo!(),
            dastore => todo!(),
            dcmpg => todo!(),
            dcmpl => todo!(),
            dconst_0 => todo!(),
            dconst_1 => todo!(),
            ddiv => todo!(),
            dload { local_index } => todo!(),
            dload_0 => todo!(),
            dload_1 => todo!(),
            dload_2 => todo!(),
            dload_3 => todo!(),
            dmul => todo!(),
            dneg => todo!(),
            drem => todo!(),
            dreturn => todo!(),
            dstore { local_index } => todo!(),
            dstore_0 => todo!(),
            dstore_1 => todo!(),
            dstore_2 => todo!(),
            dstore_3 => todo!(),
            dsub => todo!(),
            dup => todo!(),
            dup_x1 => todo!(),
            dup_x2 => todo!(),
            dup2 => todo!(),
            dup2_x1 => todo!(),
            dup2_x2 => todo!(),
            f2d => todo!(),
            f2i => todo!(),
            f2l => todo!(),
            fadd => todo!(),
            faload => todo!(),
            fastore => todo!(),
            fcmpg => todo!(),
            fcmpl => todo!(),
            fconst_0 => todo!(),
            fconst_1 => todo!(),
            fconst_2 => todo!(),
            fdiv => todo!(),
            fload { local_index } => todo!(),
            fload_0 => todo!(),
            fload_1 => todo!(),
            fload_2 => todo!(),
            fload_3 => todo!(),
            fmul => todo!(),
            fneg => todo!(),
            frem => todo!(),
            freturn => todo!(),
            fstore { local_index } => todo!(),
            fstore_0 => todo!(),
            fstore_1 => todo!(),
            fstore_2 => todo!(),
            fstore_3 => todo!(),
            fsub => todo!(),
            getfield { field_name } => todo!(),
            getstatic { field } => todo!(),
            goto(_) => todo!(),
            goto_w(_) => todo!(),
            i2b => todo!(),
            i2c => todo!(),
            i2d => todo!(),
            i2f => todo!(),
            i2l => todo!(),
            i2s => todo!(),
            iadd => todo!(),
            iaload => todo!(),
            iand => todo!(),
            iastore => todo!(),
            iconst_m1 => todo!(),
            iconst_0 => todo!(),
            iconst_1 => todo!(),
            iconst_2 => todo!(),
            iconst_3 => todo!(),
            iconst_4 => todo!(),
            iconst_5 => todo!(),
            idiv => todo!(),
            if_acmpeq(_) => todo!(),
            if_acmpne(_) => todo!(),
            if_icmpeq(_) => todo!(),
            if_icmpne(_) => todo!(),
            if_icmplt(_) => todo!(),
            if_icmpge(_) => todo!(),
            if_icmpgt(_) => todo!(),
            if_icmple(_) => todo!(),
            ifeq(_) => todo!(),
            ifne(_) => todo!(),
            iflt(_) => todo!(),
            ifge(_) => todo!(),
            ifgt(_) => todo!(),
            ifle(_) => todo!(),
            ifnonnull(_) => todo!(),
            ifnull(_) => todo!(),
            iinc { local_index, delta } => todo!(),
            iload { local_index } => todo!(),
            iload_0 => todo!(),
            iload_1 => todo!(),
            iload_2 => todo!(),
            iload_3 => todo!(),
            imul => todo!(),
            ineg => todo!(),
            instanceof { class } => todo!(),
            invokedynamic(_) => todo!(),
            invokeinterface {
                interface_method,
                count,
            } => todo!(),
            invokespecial { method_signature } => todo!(),
            invokestatic { method } => todo!(),
            invokevirtual { method_signature } => todo!(),
            ior => todo!(),
            irem => todo!(),
            ireturn => todo!(),
            ishl => todo!(),
            ishr => todo!(),
            istore { local_index } => todo!(),
            istore_0 => todo!(),
            istore_1 => todo!(),
            istore_2 => todo!(),
            istore_3 => todo!(),
            isub => todo!(),
            iushr => todo!(),
            ixor => todo!(),
            jsr(_) => todo!(),
            jsr_w(_) => todo!(),
            l2d => todo!(),
            l2f => todo!(),
            l2i => todo!(),
            ladd => todo!(),
            laload => todo!(),
            land => todo!(),
            lastore => todo!(),
            lcmp => todo!(),
            lconst_0 => todo!(),
            lconst_1 => todo!(),
            ldc(_) => todo!(),
            ldc_w(_) => todo!(),
            ldc2_w(_) => todo!(),
            ldiv => todo!(),
            lload { local_index } => todo!(),
            lload_0 => todo!(),
            lload_1 => todo!(),
            lload_2 => todo!(),
            lload_3 => todo!(),
            lmul => todo!(),
            lneg => todo!(),
            lookupswitch(_) => todo!(),
            lor => todo!(),
            lrem => todo!(),
            lreturn => todo!(),
            lshl => todo!(),
            lshr => todo!(),
            lstore { local_index } => todo!(),
            lstore_0 => todo!(),
            lstore_1 => todo!(),
            lstore_2 => todo!(),
            lstore_3 => todo!(),
            lsub => todo!(),
            lushr => todo!(),
            lxor => todo!(),
            monitorenter => todo!(),
            monitorexit => todo!(),
            multinewarray {
                item_class,
                dimensions,
            } => todo!(),
            new { class } => todo!(),
            newarray(_) => todo!(),
            nop => todo!(),
            pop => todo!(),
            pop2 => todo!(),
            putfield { field_name } => todo!(),
            putstatic { field } => todo!(),
            ret { local_index } => todo!(),
            retrn => todo!(),
            saload => todo!(),
            sastore => todo!(),
            sipush(_) => todo!(),
            swap => todo!(),
            tableswitch(_) => todo!(),
        }
    }
}

fn check_negative_array_size(size: i32) -> Result<usize, Exception> {
    if size < 0 {
        // TODO: Throw NegativeArraySizeException
        todo!()
    } else {
        Ok(size as usize)
    }
}
fn get_from_array<T: Clone>(array: &[T], index: i32) -> Result<T, Exception> {
    if index >= 0 {
        if let Some(elem) = array.get(index as usize) {
            return Ok(elem.clone());
        }
    }
    // TODO: Throw ArrayIndexOutOfBoundsException
    todo!()
}

fn put_on_array<T>(array: &mut [T], index: i32, value: T) -> OpResult {
    if index >= 0 {
        if let Some(elem) = array.get_mut(index as usize) {
            *elem = value;
            return Ok(ResultValue::None);
        }
    }
    // TODO: Throw ArrayIndexOutOfBoundsException
    todo!()
}

fn check_null<T>(nullable: Option<T>) -> Result<T, Exception> {
    if let Some(nullable) = nullable {
        Ok(nullable)
    } else {
        // TODO: Throw NullPointerException
        todo!()
    }
}

macro_rules! check_type {
    ($pattern:pat, $expression:expr) => {
        let $pattern = $expression else {
                            return Err(InternalError::WrongType);
                        };
    };
}

macro_rules! pop_stack_typechecked {
    ($pattern:pat, $stack:ident) => {
        check_type!($pattern, $stack.pop()?)
    };
}

macro_rules! get_locals_typechecked {
    ($pattern:pat, $locals:ident, $index:ident) => {
        check_type!($pattern, $locals.get_non_empty($index)?)
    };
}

macro_rules! rethrow_exception {
    ($expression:expr) => {
        match $expression {
            Ok(value) => value,
            Err(exception) => return Ok(Err(exception)),
        }
    };
}

macro_rules! type_check_nullable {
    ($pattern:pat, $expression:expr) => {
        check_type!($pattern, rethrow_exception!(check_null($expression)))
    };
}

fn exec_aaload(stack: &mut Stack) -> ExecResult {
    pop_stack_typechecked!(Object::Int(index), stack);
    pop_stack_typechecked!(Object::Array(array), stack);
    type_check_nullable!(Array::Instance(array), array);

    let array = array.lock()?;
    let reference = get_from_array(&array, index);
    drop(array);
    let reference = reference.map(Object::Instance).map(ResultValue::Object);
    Ok(reference)
}

fn exec_aastore(stack: &mut Stack) -> ExecResult {
    pop_stack_typechecked!(Object::Instance(value), stack);
    pop_stack_typechecked!(Object::Int(index), stack);
    pop_stack_typechecked!(Object::Array(array), stack);
    type_check_nullable!(Array::Instance(array), array);

    // check if value is not None that the instance if of the good class
    if let Some(instance) = &value {
        let instance_class = instance.get_class();
        let array_class = array.get_class();
        if !instance_class.is_subclass(array_class) {
            // TODO: throw ArrayStoreException
            todo!()
        }
    }

    let mut array = array.lock()?;

    Ok(put_on_array(&mut array, index, value))
}

/// Specs for all load opcodes says it *has* to be a certain type,
/// but don't specifies what to do if it's not the case
/// so let's just load whatever at that index
fn exec_load_local(locals: &Locals, index: usize, wide: bool) -> ExecResult {
    let value = if wide {
        locals.load_non_empty_wide(index)?
    } else {
        locals.load_non_empty(index)?
    };
    Ok(Ok(ResultValue::Object(value)))
}

fn exec_anewarray(stack: &mut Stack, class: &Arc<Class>) -> ExecResult {
    pop_stack_typechecked!(Object::Int(size), stack);
    let size = rethrow_exception!(check_negative_array_size(size));
    let array = Array::new_instance(class.clone(), size);
    Ok(Ok(ResultValue::Object(Object::Array(Some(array)))))
}

/// Specs for all return opcodes says it *has* to be a certain type,
/// but don't specifies what to do if it's not the case
/// so let's just return whatever on the top
fn exec_return_with_value(stack: &mut Stack) -> ExecResult {
    // don't need to clear stack, it will be dropped when function ends
    stack.pop().map(ResultValue::ReturnObject).map(Ok)
}

fn exec_arraylength(stack: &mut Stack) -> ExecResult {
    pop_stack_typechecked!(Object::Array(array), stack);
    type_check_nullable!(Array::Instance(array), array);
    array
        .size()
        .map(Object::Int)
        .map(ResultValue::Object)
        .map(Ok)
}

/// Specs for all store opcodes says it *has* to be a certain type,
/// but don't specifies what to do if it's not the case
/// so let's just store whatever at the top of the stack
fn exec_store_local(
    locals: &mut Locals,
    stack: &mut Stack,
    index: usize,
    wide: bool,
) -> ExecResult {
    let value = stack.pop()?;
    if wide {
        locals.store_wide(index, value)?;
    } else {
        locals.store(index, value)?;
    };
    Ok(Ok(ResultValue::None))
}

fn exec_athrow(stack: &mut Stack) -> ExecResult {
    pop_stack_typechecked!(Object::Instance(exception), stack);
    let exception = rethrow_exception!(check_null(exception));
    Ok(Err(exception))
}

fn exec_baload(stack: &mut Stack) -> ExecResult {
    pop_stack_typechecked!(Object::Int(index), stack);
    pop_stack_typechecked!(Object::Array(array), stack);
    let array = rethrow_exception!(check_null(array));
    let value = match array {
        Array::Boolean(array) => {
            let array = array.lock()?;
            let value = rethrow_exception!(get_from_array(&array, index));
            value.into()
        }
        Array::Byte(array) => {
            let array = array.lock()?;
            let value = rethrow_exception!(get_from_array(&array, index));
            value.into()
        }
        _ => return Err(InternalError::WrongType),
    };
    Ok(Ok(ResultValue::Object(Object::Int(value))))
}

fn exec_bastore(stack: &mut Stack) -> ExecResult {
    pop_stack_typechecked!(Object::Int(value), stack);
    pop_stack_typechecked!(Object::Int(index), stack);
    pop_stack_typechecked!(Object::Array(array), stack);
    let array = rethrow_exception!(check_null(array));
    match array {
        Array::Boolean(array) => {
            let value = if value % 2 == 0 { false } else { true };
            let mut array = array.lock()?;
            Ok(put_on_array(&mut array, index, value))
        }
        Array::Byte(array) => {
            let value = value as u8;
            let mut array = array.lock()?;
            Ok(put_on_array(&mut array, index, value))
        }
        _ => Err(InternalError::WrongType),
    }
}
