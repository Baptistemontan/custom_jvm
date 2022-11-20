use crate::parser::classfile::opcode::{ArrayType, LookupSwitch, TableSwitch};
use std::sync::Arc;



use super::{
    Array, Class, Exception, ExecResult, Field, InterfaceMethod, InternalError, Locals,
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
    load_i {
        local_index: usize,
    },
    load_0,
    load_1,
    load_2,
    load_3,
    anewarray {
        item_class: Arc<Class>,
    },
    return_v,
    arraylength,
    store_i {
        local_index: usize,
    },
    store_0,
    store_1,
    store_2,
    store_3,
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
    add,
    daload,
    dastore,
    dcmpg,
    dcmpl,
    dconst_0,
    dconst_1,
    div,
    mul,
    neg,
    rem,
    sub,
    dup,
    dup_x1,
    dup_x2,
    dup2,
    dup2_x1,
    dup2_x2,
    f2d,
    f2i,
    f2l,
    faload,
    fastore,
    fcmpg,
    fcmpl,
    fconst_0,
    fconst_1,
    fconst_2,
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
    iaload,
    iastore,
    iconst_m1,
    iconst_0,
    iconst_1,
    iconst_2,
    iconst_3,
    iconst_4,
    iconst_5,
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
    and,
    or,
    shl,
    shr,
    ushr,
    xor,
    jsr(usize),
    jsr_w(usize),
    l2d,
    l2f,
    l2i,
    laload,
    lastore,
    lcmp,
    lconst_0,
    lconst_1,
    // TODO:
    ldc(usize),
    // TODO:
    ldc_w(usize),
    ldc2_w(ConstantNumerical),
    lookupswitch(LookupSwitch),
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
            aaload => exec_aload(stack),
            aastore => exec_astore(stack),
            aconst_null => Ok(Ok(ResultValue::Object(Object::Reference(None)))), // yep that's a long wrapping for null
            load_i { local_index } => exec_load_local(locals, *local_index),
            load_0 => exec_load_local(locals, 0),
            load_1 => exec_load_local(locals, 1),
            load_2 => exec_load_local(locals, 2),
            load_3 => exec_load_local(locals, 3),
            anewarray { item_class } => exec_anewarray(stack, item_class),
            arraylength => exec_arraylength(stack),
            store_i { local_index } => exec_store_local(locals, stack, *local_index),
            store_0 => exec_store_local(locals, stack, 0),
            store_1 => exec_store_local(locals, stack, 1),
            store_2 => exec_store_local(locals, stack, 2),
            store_3 => exec_store_local(locals, stack, 3),
            athrow => exec_athrow(stack),
            baload => exec_aload(stack),
            bastore => exec_astore(stack),
            bipush(value) => Ok(Ok(ResultValue::Object(Object::Int(*value)))),
            caload => exec_aload(stack),
            castore => exec_astore(stack),
            checkcast { class } => exec_checkcast(stack, class),
            d2f => exec_d2f(stack),
            d2i => exec_d2i(stack),
            d2l => exec_d2l(stack),
            add => exec_add(stack),
            daload => exec_aload(stack),
            dastore => exec_astore(stack),
            dcmpg => exec_dcmpg(stack),
            dcmpl => exec_dcmpl(stack),
            dconst_0 => Ok(Ok(ResultValue::Object(Object::Double(0.0)))),
            dconst_1 => Ok(Ok(ResultValue::Object(Object::Double(1.0)))),
            div => exec_div(stack),
            mul => exec_mul(stack),
            rem => todo!(),
            sub => exec_sub(stack),
            dup => exec_stack_op(stack, Stack::dup),
            dup_x1 => exec_stack_op(stack, Stack::dup_x1),
            dup_x2 => exec_stack_op(stack, Stack::dup_x2),
            dup2 => exec_stack_op(stack, Stack::dup2),
            dup2_x1 => exec_stack_op(stack, Stack::dup2_x1),
            dup2_x2 => exec_stack_op(stack, Stack::dup2_x2),
            f2d => exec_f2d(stack),
            f2i => exec_f2i(stack),
            f2l => exec_f2l(stack),
            faload => exec_aload(stack),
            fastore => exec_astore(stack),
            fcmpg => exec_fcmpg(stack),
            fcmpl => exec_fcmpl(stack),
            fconst_0 => Ok(Ok(ResultValue::Object(Object::Float(0.0)))),
            fconst_1 => Ok(Ok(ResultValue::Object(Object::Float(1.0)))),
            fconst_2 => Ok(Ok(ResultValue::Object(Object::Float(2.0)))),
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
            iaload => exec_aload(stack),
            iastore => exec_astore(stack),
            iconst_m1 => Ok(Ok(ResultValue::Object(Object::Int(-1)))),
            iconst_0 => Ok(Ok(ResultValue::Object(Object::Int(0)))),
            iconst_1 => Ok(Ok(ResultValue::Object(Object::Int(1)))),
            iconst_2 => Ok(Ok(ResultValue::Object(Object::Int(2)))),
            iconst_3 => Ok(Ok(ResultValue::Object(Object::Int(3)))),
            iconst_4 => Ok(Ok(ResultValue::Object(Object::Int(4)))),
            iconst_5 => Ok(Ok(ResultValue::Object(Object::Int(5)))),
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
            instanceof { class } => exec_instanceof(stack, class),
            invokedynamic(_) => todo!(),
            invokeinterface {
                interface_method,
                count,
            } => todo!(),
            invokespecial { method_signature } => todo!(),
            invokestatic { method } => todo!(),
            invokevirtual { method_signature } => todo!(),
            neg => exec_numerical_neg(stack),
            and => exec_and(stack),
            or => exec_or(stack),
            shl => exec_shl(stack),
            shr => exec_shr(stack),
            ushr => todo!(),
            xor => exec_xor(stack),
            jsr(_) => todo!(),
            jsr_w(_) => todo!(),
            l2d => exec_l2d(stack),
            l2f => exec_l2f(stack),
            l2i => exec_l2i(stack),
            laload => todo!(),
            lastore => todo!(),
            lcmp => exec_lcmp(stack),
            lconst_0 => Ok(Ok(ResultValue::Object(Object::Long(0)))),
            lconst_1 => Ok(Ok(ResultValue::Object(Object::Long(1)))),
            ldc(_) => todo!(),
            ldc_w(_) => todo!(),
            ldc2_w(_) => todo!(),
            lookupswitch(_) => todo!(),
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
            return_v => todo!(),
        }
    }
}

fn exec_stack_op<F>(stack: &mut Stack, stack_fn: F) -> ExecResult
    where F :FnOnce(&mut Stack) -> Result<(), InternalError>
{
    stack_fn(stack).map(|_| ResultValue::None).map(Ok)
}

fn check_negative_array_size(size: i32) -> Result<usize, Exception> {
    if size < 0 {
        // TODO: Throw NegativeArraySizeException
        todo!()
    } else {
        Ok(size as usize)
    }
}

fn store_array<T>(array: &mut [T], index: i32, value: T) -> OpResult {
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

#[macro_export]
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

fn exec_aload(stack: &mut Stack) -> ExecResult {
    let index = pop_stack_typechecked!(Object::Int, stack);
    let array = pop_stack_typechecked!(Object::Array, stack);
    let array = rethrow_exception!(check_null(array));
    array.get_index(index)
}

fn exec_astore(stack: &mut Stack) -> ExecResult {
    let value = stack.pop()?;
    let index = pop_stack_typechecked!(Object::Int, stack);
    let array = pop_stack_typechecked!(Object::Array, stack);
    let array = rethrow_exception!(check_null(array));
    array.store_index(index, value)
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
    let array = rethrow_exception!(check_null(array));
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
    (exec_i2s, Object::Int, Object::Int, i16 as i32),
    (exec_l2d, Object::Long, Object::Double, f64),
    (exec_l2f, Object::Long, Object::Float, f32),
    (exec_l2i, Object::Long, Object::Int, i32)
);

macro_rules! impl_numeric_operation {
    ($(($fn_name:ident, $operation:tt)),+) => {
        $(fn $fn_name(stack: &mut Stack) -> ExecResult {
            let op_1 = stack.pop()?;
            let op_2 = stack.pop()?;
            let value = match (op_1, op_2) {
                (Object::Double(op_1), Object::Double(op_2)) => Object::Double(op_1 $operation op_2),
                (Object::Float(op_1), Object::Float(op_2)) => Object::Float(op_1 $operation op_2),
                (Object::Int(op_1), Object::Int(op_2)) => Object::Int(op_1 $operation op_2),
                (Object::Long(op_1), Object::Long(op_2)) => Object::Long(op_1 $operation op_2),
                _ => return Err(InternalError::WrongType)
            };
            Ok(Ok(ResultValue::Object(value)))
        })+
    };
}

macro_rules! impl_decimal_operation {
    ($(($fn_name:ident, $operation:tt)),+) => {
        $(fn $fn_name(stack: &mut Stack) -> ExecResult {
            let op_1 = stack.pop()?;
            let op_2 = stack.pop()?;
            let value = match (op_1, op_2) {
                (Object::Int(op_1), Object::Int(op_2)) => Object::Int(op_1 $operation op_2),
                (Object::Long(op_1), Object::Long(op_2)) => Object::Long(op_1 $operation op_2),
                _ => return Err(InternalError::WrongType)
            };
            Ok(Ok(ResultValue::Object(value)))
        })+
    };
}

impl_numeric_operation!(
    (exec_add, +),
    (exec_div, /),
    (exec_mul, *),
    (exec_sub, -)
);

impl_decimal_operation!(
    (exec_or, |),
    (exec_and, &),
    (exec_shl, <<),
    (exec_shr, >>),
    (exec_xor, ^)
);

macro_rules! impl_cmp {
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

impl_cmp!(
    (exec_dcmpg, Object::Double, 1),
    (exec_dcmpl, Object::Double, -1),
    (exec_fcmpg, Object::Float, 1),
    (exec_fcmpl, Object::Float, -1)
);

pub fn exec_lcmp(stack: &mut Stack) -> ExecResult {
    let l_1 = pop_stack_typechecked!(Object::Long, stack);
    let l_2 = pop_stack_typechecked!(Object::Long, stack);
    let value = match l_1.cmp(&l_2) {
        std::cmp::Ordering::Less => -1,
        std::cmp::Ordering::Equal => 0,
        std::cmp::Ordering::Greater => 1,
    };
    Ok(Ok(ResultValue::Object(Object::Int(value))))
}



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
            let i = pop_stack_typechecked!(Object::Int, stack);
            if i $cmp_op 0 {
                Ok(Ok(ResultValue::Jump(jump)))
            } else {
                Ok(Ok(ResultValue::None))
            }
        })+
    };
}

impl_icmp_zero!(
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
