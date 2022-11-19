use crate::parser::classfile::opcode::{ArrayType, LookupSwitch, TableSwitch};
use std::{f32::consts::E, sync::Arc};

use super::{
    reference, Array, Class, Exception, ExecResult, Field, InterfaceMethod, InternalError, Locals,
    Method, Object, OpResult, ResultValue, Stack,
};

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
            aconst_null => Ok(Ok(ResultValue::Object(Object::Reference(None)))), // yep that's a long wrapping for null
            aload { local_index } => exec_load_local(locals, *local_index),
            aload_0 => exec_load_local(locals, 0),
            aload_1 => exec_load_local(locals, 1),
            aload_2 => exec_load_local(locals, 2),
            aload_3 => exec_load_local(locals, 3),
            anewarray { item_class } => exec_anewarray(stack, item_class),
            areturn => exec_return_with_value(stack),
            arraylength => exec_arraylength(stack),
            astore { local_index } => exec_store_local(locals, stack, *local_index),
            astore_0 => exec_store_local(locals, stack, 0),
            astore_1 => exec_store_local(locals, stack, 1),
            astore_2 => exec_store_local(locals, stack, 2),
            astore_3 => exec_store_local(locals, stack, 3),
            athrow => exec_athrow(stack),
            baload => exec_baload(stack),
            bastore => exec_bastore(stack),
            bipush(value) => Ok(Ok(ResultValue::Object(Object::Int(*value)))),
            caload => exec_caload(stack),
            castore => exec_castore(stack),
            checkcast { class } => exec_checkcast(stack, class),
            d2f => exec_d2f(stack),
            d2i => exec_d2i(stack),
            d2l => exec_d2l(stack),
            dadd => exec_dadd(stack),
            daload => exec_daload(stack),
            dastore => exec_dastore(stack),
            dcmpg => exec_dcmpg(stack),
            dcmpl => exec_dcmpl(stack),
            dconst_0 => Ok(Ok(ResultValue::Object(Object::Double(0.0)))),
            dconst_1 => Ok(Ok(ResultValue::Object(Object::Double(1.0)))),
            ddiv => exec_ddiv(stack),
            dload { local_index } => exec_load_local(locals, *local_index),
            dload_0 => exec_load_local(locals, 0),
            dload_1 => exec_load_local(locals, 1),
            dload_2 => exec_load_local(locals, 2),
            dload_3 => exec_load_local(locals, 3),
            dmul => exec_dmul(stack),
            dneg => exec_numerical_neg(stack),
            drem => todo!(),
            dreturn => exec_return_with_value(stack),
            dstore { local_index } => exec_store_local(locals, stack, *local_index),
            dstore_0 => exec_store_local(locals, stack, 0),
            dstore_1 => exec_store_local(locals, stack, 1),
            dstore_2 => exec_store_local(locals, stack, 2),
            dstore_3 => exec_store_local(locals, stack, 3),
            dsub => exec_dsub(stack),
            dup => stack.dup().map(|_| ResultValue::None).map(Ok),
            dup_x1 => stack.dup_x1().map(|_| ResultValue::None).map(Ok),
            dup_x2 => stack.dup_x2().map(|_| ResultValue::None).map(Ok),
            dup2 => stack.dup2().map(|_| ResultValue::None).map(Ok),
            dup2_x1 => stack.dup2_x1().map(|_| ResultValue::None).map(Ok),
            dup2_x2 => stack.dup2_x2().map(|_| ResultValue::None).map(Ok),
            f2d => exec_f2d(stack),
            f2i => exec_f2i(stack),
            f2l => exec_f2l(stack),
            fadd => exec_fadd(stack),
            faload => exec_faload(stack),
            fastore => exec_fastore(stack),
            fcmpg => exec_fcmpg(stack),
            fcmpl => exec_fcmpl(stack),
            fconst_0 => Ok(Ok(ResultValue::Object(Object::Float(0.0)))),
            fconst_1 => Ok(Ok(ResultValue::Object(Object::Float(1.0)))),
            fconst_2 => Ok(Ok(ResultValue::Object(Object::Float(2.0)))),
            fdiv => exec_fdiv(stack),
            fload { local_index } => exec_load_local(locals, *local_index),
            fload_0 => exec_load_local(locals, 0),
            fload_1 => exec_load_local(locals, 1),
            fload_2 => exec_load_local(locals, 2),
            fload_3 => exec_load_local(locals, 3),
            fmul => exec_fmul(stack),
            fneg => exec_numerical_neg(stack),
            frem => todo!(),
            freturn => exec_return_with_value(stack),
            fstore { local_index } => exec_store_local(locals, stack, *local_index),
            fstore_0 => exec_store_local(locals, stack, 0),
            fstore_1 => exec_store_local(locals, stack, 1),
            fstore_2 => exec_store_local(locals, stack, 2),
            fstore_3 => exec_store_local(locals, stack, 3),
            fsub => exec_fsub(stack),
            getfield { field_name } => todo!(),
            getstatic { field } => todo!(),
            goto(jump) => Ok(Ok(ResultValue::Jump(*jump))),
            goto_w(jump) => Ok(Ok(ResultValue::Jump(*jump))),
            i2b => exec_i2b(stack),
            i2c => exec_i2c(stack),
            i2d => exec_i2d(stack),
            i2f => exec_i2f(stack),
            i2l => exec_i2l(stack),
            i2s => exec_i2s(stack),
            iadd => exec_iadd(stack),
            iaload => exec_iaload(stack),
            iand => exec_iand(stack),
            iastore => exec_iastore(stack),
            iconst_m1 => Ok(Ok(ResultValue::Object(Object::Int(-1)))),
            iconst_0 => Ok(Ok(ResultValue::Object(Object::Int(0)))),
            iconst_1 => Ok(Ok(ResultValue::Object(Object::Int(1)))),
            iconst_2 => Ok(Ok(ResultValue::Object(Object::Int(2)))),
            iconst_3 => Ok(Ok(ResultValue::Object(Object::Int(3)))),
            iconst_4 => Ok(Ok(ResultValue::Object(Object::Int(4)))),
            iconst_5 => Ok(Ok(ResultValue::Object(Object::Int(5)))),
            idiv => exec_idiv(stack),
            if_acmpeq(jump) => exec_if_acmpeq(stack, *jump),
            if_acmpne(jump) => exec_if_acmpne(stack, *jump),
            if_icmpeq(jump) => exec_if_icmpeq(stack, *jump),
            if_icmpne(jump) => exec_if_icmpne(stack, *jump),
            if_icmplt(jump) => exec_if_icmplt(stack, *jump),
            if_icmpge(jump) => exec_if_icmpge(stack, *jump),
            if_icmpgt(jump) => exec_if_icmpgt(stack, *jump),
            if_icmple(jump) => exec_if_icmple(stack, *jump),
            ifeq(jump) => exec_ifeq(stack, *jump),
            ifne(jump) => exec_ifne(stack, *jump),
            iflt(jump) => exec_iflt(stack, *jump),
            ifge(jump) => exec_ifge(stack, *jump),
            ifgt(jump) => exec_ifgt(stack, *jump),
            ifle(jump) => exec_ifle(stack, *jump),
            ifnonnull(jump) => exec_ifnonnull(stack, *jump),
            ifnull(jump) => exec_ifnull(stack, *jump),
            iinc { local_index, delta } => exec_iinc(locals, *local_index, *delta),
            iload { local_index } => exec_load_local(locals, *local_index),
            iload_0 => exec_load_local(locals, 0),
            iload_1 => exec_load_local(locals, 1),
            iload_2 => exec_load_local(locals, 2),
            iload_3 => exec_load_local(locals, 3),
            imul => exec_imul(stack),
            ineg => exec_numerical_neg(stack),
            instanceof { class } => exec_instanceof(stack, class),
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
    ($pattern:path, $expression:expr) => {{
        if let $pattern(value) = $expression {
            value
        } else {
            return Err(InternalError::WrongType);
        }
    }};
}

macro_rules! pop_stack_typechecked {
    ($pattern:path, $stack:ident) => {{
        check_type!($pattern, $stack.pop()?)
    }};
}

macro_rules! get_locals_typechecked {
    ($pattern:pat, $locals:ident, $index:ident) => {{
        check_type!($pattern, $locals.get_non_empty($index)?)
    }};
}

macro_rules! rethrow_exception {
    ($expression:expr) => {{
        match $expression {
            Ok(value) => value,
            Err(exception) => return Ok(Err(exception)),
        }
    }};
}

macro_rules! type_check_nullable {
    ($pattern:path, $expression:expr) => {
        check_type!($pattern, rethrow_exception!(check_null($expression)))
    };
}

// Some comments will talk about the specs saying something *must* be of this type or whatever
// The requirements should be met by the compiler, but in the case the requirement are not met
// the behavior is undefined by the specs and can be implemented as we like.
// For some opcodes the error will be silently ignored (like loading a local or returning a value from function)
// but some other opcodes can't progress if those requirement are met (like storing in an array, as those have a defined type)
// so an InternalError will be returned.
// There is a good chance that a silently ignored error can lead to an internal error afterward
// like loading a local of a wrong type, then trying to store it in array.
// I will try to put a comment every time I encounter a possible UB describing the implemented behavior

fn exec_aaload(stack: &mut Stack) -> ExecResult {
    let index = pop_stack_typechecked!(Object::Int, stack);
    let array = pop_stack_typechecked!(Object::Array, stack);
    let array = type_check_nullable!(Array::Reference, array);

    let array = array.lock()?;
    let reference = get_from_array(&array, index);
    drop(array);
    let reference = reference.map(Object::Reference).map(ResultValue::Object);
    Ok(reference)
}

fn exec_aastore(stack: &mut Stack) -> ExecResult {
    let value = pop_stack_typechecked!(Object::Reference, stack);
    let index = pop_stack_typechecked!(Object::Int, stack);
    let array = pop_stack_typechecked!(Object::Array, stack);
    let array = type_check_nullable!(Array::Reference, array);

    // check if value is not None that the Reference if of the good class
    if let Some(Reference) = &value {
        let Reference_class = Reference.get_class();
        let array_class = array.get_class();
        if !Reference_class.is_subclass(array_class) {
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
fn exec_load_local(locals: &Locals, index: usize) -> ExecResult {
    locals
        .load_non_empty(index)
        .map(ResultValue::Object)
        .map(Ok)
}

fn exec_anewarray(stack: &mut Stack, class: &Arc<Class>) -> ExecResult {
    let size = pop_stack_typechecked!(Object::Int, stack);
    let size = rethrow_exception!(check_negative_array_size(size));
    let array = Array::new_reference(class.clone(), size);
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
    let array = pop_stack_typechecked!(Object::Array, stack);
    let array = type_check_nullable!(Array::Reference, array);
    array
        .size()
        .map(Object::Int)
        .map(ResultValue::Object)
        .map(Ok)
}

/// Specs for all store opcodes says it *has* to be a certain type,
/// but don't specifies what to do if it's not the case
/// so let's just store whatever at the top of the stack
fn exec_store_local(locals: &mut Locals, stack: &mut Stack, index: usize) -> ExecResult {
    let value = stack.pop()?;
    locals.store(index, value)?;
    Ok(Ok(ResultValue::None))
}

fn exec_athrow(stack: &mut Stack) -> ExecResult {
    let exception = pop_stack_typechecked!(Object::Reference, stack);
    let exception = rethrow_exception!(check_null(exception));
    Ok(Err(exception))
}

fn exec_baload(stack: &mut Stack) -> ExecResult {
    let index = pop_stack_typechecked!(Object::Int, stack);
    let array = pop_stack_typechecked!(Object::Array, stack);
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
        // Specs says it *must* be of type bool or byte,
        // but don't specifies what to do in the case it is not
        // We can't just load whatever (could load default, surely not what the user want)
        // so let's return error
        _ => return Err(InternalError::WrongType),
    };
    Ok(Ok(ResultValue::Object(Object::Int(value))))
}

fn exec_bastore(stack: &mut Stack) -> ExecResult {
    let value = pop_stack_typechecked!(Object::Int, stack);
    let index = pop_stack_typechecked!(Object::Int, stack);
    let array = pop_stack_typechecked!(Object::Array, stack);
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
        // same as baload, but in the store case we could silently ignore the wrong array type
        // and just not store anything, but that is surely not something that should happend,
        // so InteralError it is
        _ => Err(InternalError::WrongType),
    }
}

macro_rules! impl_numerical_aload {
    ($(($fn_name:ident, $array_type:path, $value_type:path)),+) => {
        $(fn $fn_name(stack: &mut Stack) -> ExecResult {
            let index = pop_stack_typechecked!(Object::Int, stack);
            let array = pop_stack_typechecked!(Object::Array, stack);
            let array = type_check_nullable!($array_type, array);
            let array = array.lock()?;
            let value = rethrow_exception!(get_from_array(&array, index));
            Ok(Ok(ResultValue::Object($value_type(value.into()))))
        })+
    };
}

impl_numerical_aload!(
    (exec_caload, Array::Char, Object::Int),
    (exec_daload, Array::Double, Object::Double),
    (exec_faload, Array::Float, Object::Float),
    (exec_iaload, Array::Int, Object::Int)
);

macro_rules! impl_numerical_astore {
    ($(($fn_name:ident, $value:path, $array_type:path, $cast:tt)),+) => {
        $(fn $fn_name(stack: &mut Stack) -> ExecResult {
            let value = pop_stack_typechecked!($value, stack);
            let index = pop_stack_typechecked!(Object::Int, stack);
            let array = pop_stack_typechecked!(Object::Array, stack);
            let array = type_check_nullable!($array_type, array);
            let mut array = array.lock()?;
            Ok(put_on_array(&mut array, index, value as $cast))
        })+
    };
}

impl_numerical_astore!(
    (exec_castore, Object::Int, Array::Char, u8),
    (exec_dastore, Object::Double, Array::Double, f64),
    (exec_fastore, Object::Float, Array::Float, f32),
    (exec_iastore, Object::Int, Array::Int, i32)
);

fn exec_checkcast(stack: &mut Stack, super_class: &Arc<Class>) -> ExecResult {
    let reference = pop_stack_typechecked!(Object::Reference, stack);
    if let Some(reference) = reference {
        if reference.is_subclass(super_class) {
            Ok(Ok(ResultValue::Object(Object::Reference(Some(reference)))))
        } else {
            // TODO: Throw ClassCastException
            todo!()
        }
    } else {
        Ok(Ok(ResultValue::Object(Object::Reference(None))))
    }
}

macro_rules! impl_numerical_cast {
    ($(($fn_name:ident, $from:path, $to:path, $($cast:tt)+)), +) => {
        $(fn $fn_name(stack: &mut Stack) -> ExecResult {
            let value = pop_stack_typechecked!($from, stack);
            Ok(Ok(ResultValue::Object($to(value as $($cast)+))))
        })+
    };
}

impl_numerical_cast!(
    (exec_d2f, Object::Double, Object::Float, f32),
    (exec_d2i, Object::Double, Object::Int, i32),
    (exec_d2l, Object::Double, Object::Long, i64),
    (exec_f2d, Object::Float, Object::Double, f64),
    (exec_f2i, Object::Float, Object::Int, i32),
    (exec_f2l, Object::Float, Object::Long, i64),
    (exec_i2b, Object::Int, Object::Int, u8 as i32),
    (exec_i2c, Object::Int, Object::Int, u8 as i32),
    (exec_i2d, Object::Int, Object::Double, f64),
    (exec_i2f, Object::Int, Object::Float, f32),
    (exec_i2l, Object::Int, Object::Long, i64),
    (exec_i2s, Object::Int, Object::Int, i16 as i32)
);

macro_rules! impl_numeric_operation {
    ($(($fn_name:ident, $num_type:path, $operation:tt)),+) => {
        $(fn $fn_name(stack: &mut Stack) -> ExecResult {
            let op_1 = pop_stack_typechecked!($num_type, stack);
            let op_2 = pop_stack_typechecked!($num_type, stack);
            Ok(Ok(ResultValue::Object($num_type(op_1 $operation op_2))))
        })+
    };
}

impl_numeric_operation!(
    (exec_dadd, Object::Double, +),
    (exec_ddiv, Object::Double, /),
    (exec_dmul, Object::Double, *),
    (exec_dsub, Object::Double, -),
    (exec_fadd, Object::Float, +),
    (exec_fdiv, Object::Float, /),
    (exec_fmul, Object::Float, *),
    (exec_fsub, Object::Float, -),
    (exec_iadd, Object::Int, +),
    (exec_idiv, Object::Int, /),
    (exec_imul, Object::Int, *),
    (exec_isub, Object::Int, -),
    (exec_iand, Object::Int, &)
);

macro_rules! impl_float_cmp {
    ($(($fn_name:ident, $num_type:path, $default:literal)), +) => {
        $(fn $fn_name(stack: &mut Stack) -> ExecResult {
            let op_1 = pop_stack_typechecked!($num_type, stack);
            let op_2 = pop_stack_typechecked!($num_type, stack);
            let value = match op_1.partial_cmp(&op_2) {
                Some(std::cmp::Ordering::Greater) => 1,
                Some(std::cmp::Ordering::Equal) => 0,
                Some(std::cmp::Ordering::Less) => -1,
                None => $default,
            };
            Ok(Ok(ResultValue::Object(Object::Int(value))))
        })+
    };
}

impl_float_cmp!(
    (exec_dcmpg, Object::Double, 1),
    (exec_dcmpl, Object::Double, -1),
    (exec_fcmpg, Object::Float, 1),
    (exec_fcmpl, Object::Float, -1)
);

fn exec_numerical_neg(stack: &mut Stack) -> ExecResult {
    let num = stack.pop()?;
    let num = match num {
        Object::Int(num) => Object::Int(-num),
        Object::Float(num) => Object::Float(-num),
        Object::Long(num) => Object::Long(-num),
        Object::Double(num) => Object::Double(-num),
        _ => return Err(InternalError::WrongType),
    };
    Ok(Ok(ResultValue::Object(num)))
}

fn exec_if_acmpeq(stack: &mut Stack, jump: usize) -> ExecResult {
    let ref1 = pop_stack_typechecked!(Object::Reference, stack);
    let ref2 = pop_stack_typechecked!(Object::Reference, stack);
    if ref1 == ref2 {
        Ok(Ok(ResultValue::Jump(jump)))
    } else {
        Ok(Ok(ResultValue::None))
    }
}

fn exec_if_acmpne(stack: &mut Stack, jump: usize) -> ExecResult {
    let ref_1 = pop_stack_typechecked!(Object::Reference, stack);
    let ref_2 = pop_stack_typechecked!(Object::Reference, stack);
    if ref_1 != ref_2 {
        Ok(Ok(ResultValue::Jump(jump)))
    } else {
        Ok(Ok(ResultValue::None))
    }
}

macro_rules! impl_icmp {
    ($(($fn_name:ident, $cmp_op:tt)), +) => {
        $(fn $fn_name(stack: &mut Stack, jump: usize) -> ExecResult {
            let i_1 = pop_stack_typechecked!(Object::Int, stack);
            let i_2 = pop_stack_typechecked!(Object::Int, stack);
            if i_1 $cmp_op i_2 {
                Ok(Ok(ResultValue::Jump(jump)))
            } else {
                Ok(Ok(ResultValue::None))
            }
        })+
    };
}

impl_icmp!(
    (exec_if_icmpeq, ==),
    (exec_if_icmpne, !=),
    (exec_if_icmplt, < ),
    (exec_if_icmpge, >=),
    (exec_if_icmpgt, > ),
    (exec_if_icmple, <=)
);

macro_rules! impl_icmp_zero {
    ($(($fn_name:ident, $cmp_op:tt)), +) => {
        $(fn $fn_name(stack: &mut Stack, jump: usize) -> ExecResult {
            pop_stack_typechecked!(Object::Int(i), stack);
            if i1 $cmp_op 0 {
                Ok(Ok(ResultValue::Jump(jump)))
            } else {
                Ok(Ok(ResultValue::None))
            }
        })+
    };
}

impl_icmp!(
    (exec_ifeq, ==),
    (exec_ifne, !=),
    (exec_iflt, < ),
    (exec_ifge, >=),
    (exec_ifgt, > ),
    (exec_ifle, <=)
);

fn exec_ifnonnull(stack: &mut Stack, jump: usize) -> ExecResult {
    let reference = pop_stack_typechecked!(Object::Reference, stack);
    if reference.is_some() {
        Ok(Ok(ResultValue::Jump(jump)))
    } else {
        Ok(Ok(ResultValue::None))
    }
}

fn exec_ifnull(stack: &mut Stack, jump: usize) -> ExecResult {
    let reference = pop_stack_typechecked!(Object::Reference, stack);
    if reference.is_none() {
        Ok(Ok(ResultValue::Jump(jump)))
    } else {
        Ok(Ok(ResultValue::None))
    }
}

fn exec_iinc(locals: &mut Locals, index: usize, delta: i32) -> ExecResult {
    let i = locals.get_non_empty_mut(index)?;
    let i = check_type!(Object::Int, i);
    *i += delta;
    Ok(Ok(ResultValue::None))
}

fn exec_instanceof(stack: &mut Stack, class: &Arc<Class>) -> ExecResult {
    let reference = pop_stack_typechecked!(Object::Reference, stack);
    let value = if let Some(reference) = reference {
        if reference.is_subclass(class) {
            1
        } else {
            0
        }
    } else {
        0
    };
    Ok(Ok(ResultValue::Object(Object::Int(value))))
}
