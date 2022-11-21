use std::{cell::RefCell, collections::HashMap, sync::Arc};

use crate::{
    parser::utils::{
        self, pop1, pop4, pop_u1_as_index, pop_u2_as_index, pop_u2_as_offset, pop_u4_as_index,
        pop_u4_as_offset, skip_n, FileByte, ParseError,
    },
    runtime_types::Class,
};

#[derive(Debug, Clone)]
pub enum ArrayType {
    Boolean,
    Char,
    Float,
    Double,
    Byte,
    Short,
    Int,
    Long,
    // anewarray transform to newarray with ArrayType = Reference
    Reference(Arc<Class>),
}

impl TryFrom<u8> for ArrayType {
    type Error = ParseError;

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        use ArrayType::*;
        match value {
            4 => Ok(Boolean),
            5 => Ok(Char),
            6 => Ok(Float),
            7 => Ok(Double),
            8 => Ok(Byte),
            9 => Ok(Short),
            10 => Ok(Int),
            11 => Ok(Long),
            _ => Err(ParseError::InvalidOpCode),
        }
    }
}

#[derive(Debug, Clone)]
#[allow(non_camel_case_types)]
pub enum OpCode {
    aaload,                        // 0x32
    aastore,                       // 0x53
    aconst_null,                   // 0x01
    aload(usize),                  // 0x19
    aload_0,                       // 0x2a
    aload_1,                       // 0x2b
    aload_2,                       // 0x2c
    aload_3,                       // 0x2d
    anewarray(usize),              // 0xbd
    areturn,                       // 0xb0
    arraylength,                   // 0xbe
    astore(usize),                 // 0x3a
    astore_0,                      // 0x4b
    astore_1,                      // 0x4c
    astore_2,                      // 0x4d
    astore_3,                      // 0x4e
    athrow,                        // 0xbf
    baload,                        // 0x33
    bastore,                       // 0x54
    bipush(i32),                   // 0x10
    caload,                        // 0x34
    castore,                       // 0x55
    checkcast(usize),              // 0xc0
    d2f,                           // 0x90
    d2i,                           // 0x8e
    d2l,                           // 0x8f
    dadd,                          // 0x63
    daload,                        // 0x31
    dastore,                       // 0x52
    dcmpg,                         // 0x98
    dcmpl,                         // 0x97
    dconst_0,                      // 0x0e
    dconst_1,                      // 0x0f
    ddiv,                          // 0x6f
    dload(usize),                  // 0x18
    dload_0,                       // 0x26
    dload_1,                       // 0x27
    dload_2,                       // 0x28
    dload_3,                       // 0x29
    dmul,                          // 0x6b
    dneg,                          // 0x77
    drem,                          // 0x73
    dreturn,                       // 0xaf
    dstore(usize),                 // 0x39
    dstore_0,                      // 0x47
    dstore_1,                      // 0x48
    dstore_2,                      // 0x49
    dstore_3,                      // 0x4a
    dsub,                          // 0x67
    dup,                           // 0x59
    dup_x1,                        // 0x5a
    dup_x2,                        // 0x5b
    dup2,                          // 0x5c
    dup2_x1,                       // 0x5d
    dup2_x2,                       // 0x5e
    f2d,                           // 0x8d
    f2i,                           // 0x8b
    f2l,                           // 0x8c
    fadd,                          // 0x62
    faload,                        // 0x30
    fastore,                       // 0x51
    fcmpg,                         // 0x96
    fcmpl,                         // 0x95
    fconst_0,                      // 0x0b
    fconst_1,                      // 0x0c
    fconst_2,                      // 0x0d
    fdiv,                          // 0x6e
    fload(usize),                  // 0x17
    fload_0,                       // 0x22
    fload_1,                       // 0x23
    fload_2,                       // 0x24
    fload_3,                       // 0x25
    fmul,                          // 0x6a
    fneg,                          // 0x76
    frem,                          // 0x72
    freturn,                       // 0xae
    fstore(usize),                 // 0x38
    fstore_0,                      // 0x43
    fstore_1,                      // 0x44
    fstore_2,                      // 0x45
    fstore_3,                      // 0x46
    fsub,                          // 0x66
    getfield(usize),               // 0xb4
    getstatic(usize),              // 0xb2
    goto(usize),                   // 0xa7
    goto_w(usize),                 // 0xc8
    i2b,                           // 0x91
    i2c,                           // 0x92
    i2d,                           // 0x87
    i2f,                           // 0x86
    i2l,                           // 0x85
    i2s,                           // 0x93
    iadd,                          // 0x60
    iaload,                        // 0x2e
    iand,                          // 0x7e
    iastore,                       // 0x4f
    iconst_m1,                     // 0x02
    iconst_0,                      // 0x03
    iconst_1,                      // 0x04
    iconst_2,                      // 0x05
    iconst_3,                      // 0x06
    iconst_4,                      // 0x07
    iconst_5,                      // 0x08
    idiv,                          // 0x6c
    if_acmpeq(usize),              // 0xa5
    if_acmpne(usize),              // 0xa6
    if_icmpeq(usize),              // 0x9f
    if_icmpne(usize),              // 0xa0
    if_icmplt(usize),              // 0xa1
    if_icmpge(usize),              // 0xa2
    if_icmpgt(usize),              // 0xa3
    if_icmple(usize),              // 0xa4
    ifeq(usize),                   // 0x99
    ifne(usize),                   // 0x9a
    iflt(usize),                   // 0x9b
    ifge(usize),                   // 0x9c
    ifgt(usize),                   // 0x9d
    ifle(usize),                   // 0x9e
    ifnonnull(usize),              // 0xc7
    ifnull(usize),                 // 0xc6
    iinc(usize, i32),              // 0x84
    iload(usize),                  // 0x15
    iload_0,                       // 0x1a
    iload_1,                       // 0x1b
    iload_2,                       // 0x1c
    iload_3,                       // 0x1d
    imul,                          // 0x68
    ineg,                          // 0x74
    instanceof(usize),             // 0xc1
    invokedynamic(usize),          // 0xba
    invokeinterface(usize, usize), // 0xb9
    invokespecial(usize),          // 0xb7
    invokestatic(usize),           // 0xb8
    invokevirtual(usize),          // 0xb6
    ior,                           // 0x80
    irem,                          // 0x70
    ireturn,                       // 0xac
    ishl,                          // 0x78
    ishr,                          // 0x7a
    istore(usize),                 // 0x36
    istore_0,                      // 0x3b
    istore_1,                      // 0x3c
    istore_2,                      // 0x3d
    istore_3,                      // 0x3e
    isub,                          // 0x64
    iushr,                         // 0x7c
    ixor,                          // 0x82
    jsr(usize),                    // 0xa8
    jsr_w(usize),                  // 0xc9
    l2d,                           // 0x8a
    l2f,                           // 0x89
    l2i,                           // 0x88
    ladd,                          // 0x61
    laload,                        // 0x2f
    land,                          // 0x7f
    lastore,                       // 0x50
    lcmp,                          // 0x94
    lconst_0,                      // 0x09
    lconst_1,                      // 0x0a
    ldc(usize),                    // 0x12
    ldc_w(usize),                  // 0x13
    ldc2_w(usize),                 // 0x14
    ldiv,                          // 0x6d
    lload(usize),                  // 0x16
    lload_0,                       // 0x1e
    lload_1,                       // 0x1f
    lload_2,                       // 0x20
    lload_3,                       // 0x21
    lmul,                          // 0x69
    lneg,                          // 0x75
    lookupswitch(LookupSwitch),    // 0xab
    lor,                           // 0x81
    lrem,                          // 0x71
    lreturn,                       // 0xad
    lshl,                          // 0x79
    lshr,                          // 0x7b
    lstore(usize),                 // 0x37
    lstore_0,                      // 0x3f
    lstore_1,                      // 0x40
    lstore_2,                      // 0x41
    lstore_3,                      // 0x42
    lsub,                          // 0x65
    lushr,                         // 0x7d
    lxor,                          // 0x83
    monitorenter,                  // 0xc2
    monitorexit,                   // 0xc3
    multinewarray(usize, usize),   // 0xc5
    new(usize),                    // 0xbb
    newarray(ArrayType),           // 0xbc
    nop,                           // 0x00
    pop,                           // 0x57
    pop2,                          // 0x58
    putfield(usize),               // 0xb5
    putstatic(usize),              // 0xb3
    ret(usize),                    // 0xa9
    retrn,                         // 0xb1 | return
    saload,                        // 0x35
    sastore,                       // 0x56
    sipush(i32),                   // 0x11
    swap,                          // 0x5f
    tableswitch(TableSwitch),      // 0xaa
    wide(Wide),                    // 0xc4
}

fn parse_u2_index_offset<I>(bytes: &mut I, current_line: usize) -> Result<usize, ParseError>
where
    I: Iterator<Item = FileByte>,
{
    let offset = pop_u2_as_offset(bytes)?;
    let abs_offset = offset.abs() as usize;
    if offset < 0 {
        Ok(current_line - abs_offset)
    } else {
        Ok(current_line + abs_offset)
    }
}

fn parse_u4_index_offset<I>(bytes: &mut I, current_line: usize) -> Result<usize, ParseError>
where
    I: Iterator<Item = FileByte>,
{
    let offset = pop_u4_as_offset(bytes)?;
    let abs_offset = offset.abs() as usize;
    if offset < 0 {
        Ok(current_line - abs_offset)
    } else {
        Ok(current_line + abs_offset)
    }
}

fn parse_opcode<I>(bytes: &mut I, current_line: usize) -> Result<OpCode, ParseError>
where
    I: Iterator<Item = FileByte>,
{
    use OpCode::*;
    let tag = pop1(bytes)?;
    let op_code = match tag {
        0x32 => OpCode::aaload,
        0x53 => OpCode::aastore,
        0x01 => aconst_null,
        0x19 => {
            let index = pop_u1_as_index(bytes)?;
            aload(index)
        }
        0x2a => aload_0,
        0x2b => aload_1,
        0x2c => aload_2,
        0x2d => aload_3,
        0xbd => {
            let index = pop_u2_as_index(bytes)?;
            anewarray(index)
        }
        0xb0 => areturn,
        0xbe => arraylength,
        0x3a => {
            let index = pop_u1_as_index(bytes)?;
            astore(index)
        }
        0x4b => astore_0,
        0x4c => astore_1,
        0x4d => astore_2,
        0x4e => astore_3,
        0xbf => athrow,
        0x33 => baload,
        0x54 => bastore,
        0x10 => {
            let byte = pop1(bytes)?;
            let value = i8::from_be_bytes([byte]);
            bipush(value.into())
        }
        0x34 => caload,
        0x55 => castore,
        0xc0 => {
            let index = pop_u2_as_index(bytes)?;
            checkcast(index)
        }
        0x90 => d2f,
        0x8e => d2i,
        0x8f => d2l,
        0x63 => dadd,
        0x31 => daload,
        0x52 => dastore,
        0x98 => dcmpg,
        0x97 => dcmpl,
        0x0e => dconst_0,
        0x0f => dconst_1,
        0x6f => ddiv,
        0x18 => {
            let index = pop_u1_as_index(bytes)?;
            dload(index)
        }
        0x26 => dload_0,
        0x27 => dload_1,
        0x28 => dload_2,
        0x29 => dload_3,
        0x6b => dmul,
        0x77 => dneg,
        0x73 => drem,
        0xaf => dreturn,
        0x39 => {
            let index = pop_u1_as_index(bytes)?;
            dstore(index)
        }
        0x47 => dstore_0,
        0x48 => dstore_1,
        0x49 => dstore_2,
        0x4a => dstore_3,
        0x67 => dsub,
        0x59 => dup,
        0x5a => dup_x1,
        0x5b => dup_x2,
        0x5c => dup2,
        0x5d => dup2_x1,
        0x5e => dup2_x2,
        0x8d => f2d,
        0x8b => f2i,
        0x8c => f2l,
        0x62 => fadd,
        0x30 => faload,
        0x51 => fastore,
        0x96 => fcmpg,
        0x95 => fcmpl,
        0x0b => fconst_0,
        0x0c => fconst_1,
        0x0d => fconst_2,
        0x6e => fdiv,
        0x17 => {
            let index = pop_u1_as_index(bytes)?;
            fload(index)
        }
        0x22 => fload_0,
        0x23 => fload_1,
        0x24 => fload_2,
        0x25 => fload_3,
        0x6a => fmul,
        0x76 => fneg,
        0x72 => frem,
        0xae => freturn,
        0x38 => {
            let index = pop_u1_as_index(bytes)?;
            fstore(index)
        }
        0x43 => fstore_0,
        0x44 => fstore_1,
        0x45 => fstore_2,
        0x46 => fstore_3,
        0x66 => fsub,
        0xb4 => {
            let index = pop_u2_as_index(bytes)?;
            getfield(index)
        }
        0xb2 => {
            let index = pop_u2_as_index(bytes)?;
            getstatic(index)
        }
        0xa7 => {
            let target_line = parse_u2_index_offset(bytes, current_line)?;
            goto(target_line)
        }
        0xc8 => {
            let target_line = parse_u4_index_offset(bytes, current_line)?;
            goto_w(target_line)
        }
        0x91 => i2b,
        0x92 => i2c,
        0x87 => i2d,
        0x86 => i2f,
        0x85 => i2l,
        0x93 => i2s,
        0x60 => iadd,
        0x2e => iaload,
        0x7e => iand,
        0x4f => iastore,
        0x02 => iconst_m1,
        0x03 => iconst_0,
        0x04 => iconst_1,
        0x05 => iconst_2,
        0x06 => iconst_3,
        0x07 => iconst_4,
        0x08 => iconst_5,
        0x6c => idiv,
        0xa5 => {
            let target_line = parse_u2_index_offset(bytes, current_line)?;
            if_acmpeq(target_line)
        }
        0xa6 => {
            let target_line = parse_u2_index_offset(bytes, current_line)?;
            if_acmpne(target_line)
        }
        0x9f => {
            let target_line = parse_u2_index_offset(bytes, current_line)?;
            if_icmpeq(target_line)
        }
        0xa0 => {
            let target_line = parse_u2_index_offset(bytes, current_line)?;
            if_icmpne(target_line)
        }
        0xa1 => {
            let target_line = parse_u2_index_offset(bytes, current_line)?;
            if_icmplt(target_line)
        }
        0xa2 => {
            let target_line = parse_u2_index_offset(bytes, current_line)?;
            if_icmpge(target_line)
        }
        0xa3 => {
            let target_line = parse_u2_index_offset(bytes, current_line)?;
            if_icmpgt(target_line)
        }
        0xa4 => {
            let target_line = parse_u2_index_offset(bytes, current_line)?;
            if_icmple(target_line)
        }
        0x99 => {
            let target_line = parse_u2_index_offset(bytes, current_line)?;
            ifeq(target_line)
        }
        0x9a => {
            let target_line = parse_u2_index_offset(bytes, current_line)?;
            ifne(target_line)
        }
        0x9b => {
            let target_line = parse_u2_index_offset(bytes, current_line)?;
            iflt(target_line)
        }
        0x9c => {
            let target_line = parse_u2_index_offset(bytes, current_line)?;
            ifge(target_line)
        }
        0x9d => {
            let target_line = parse_u2_index_offset(bytes, current_line)?;
            ifgt(target_line)
        }
        0x9e => {
            let target_line = parse_u2_index_offset(bytes, current_line)?;
            ifle(target_line)
        }
        0xc7 => {
            let target_line = parse_u2_index_offset(bytes, current_line)?;
            ifnonnull(target_line)
        }
        0xc6 => {
            let target_line = parse_u2_index_offset(bytes, current_line)?;
            ifnull(target_line)
        }
        0x84 => {
            let index = pop_u1_as_index(bytes)?;
            let delta_bits = pop1(bytes)?;
            let delta = i8::from_be_bytes([delta_bits]).into();
            iinc(index, delta)
        }
        0x15 => {
            let index = pop_u1_as_index(bytes)?;
            iload(index)
        }
        0x1a => iload_0,
        0x1b => iload_1,
        0x1c => iload_2,
        0x1d => iload_3,
        0x68 => imul,
        0x74 => ineg,
        0xc1 => {
            let index = pop_u2_as_index(bytes)?;
            instanceof(index)
        }
        0xba => {
            let index = pop_u2_as_index(bytes)?;
            let padded_bits = pop_u2_as_index(bytes)?;
            if padded_bits != 0 {
                return Err(ParseError::InvalidOpCode);
            }
            invokedynamic(index)
        }
        0xb9 => {
            let index = pop_u2_as_index(bytes)?;
            let count = pop1(bytes)?;
            let padding = pop1(bytes)?;
            if count == 0 || padding != 0 {
                return Err(ParseError::InvalidOpCode);
            }
            invokeinterface(index, count.into())
        }
        0xb7 => {
            let index = pop_u2_as_index(bytes)?;
            invokespecial(index)
        }
        0xb8 => {
            let index = pop_u2_as_index(bytes)?;
            invokestatic(index)
        }
        0xb6 => {
            let index = pop_u2_as_index(bytes)?;
            invokevirtual(index)
        }
        0x80 => ior,
        0x70 => irem,
        0xac => ireturn,
        0x78 => ishl,
        0x7a => ishr,
        0x36 => {
            let index = pop_u1_as_index(bytes)?;
            istore(index)
        }
        0x3b => istore_0,
        0x3c => istore_1,
        0x3d => istore_2,
        0x3e => istore_3,
        0x64 => isub,
        0x7c => iushr,
        0x82 => ixor,
        0xa8 => {
            let target_line = parse_u2_index_offset(bytes, current_line)?;
            jsr(target_line)
        }
        0xc9 => {
            let target_line = parse_u4_index_offset(bytes, current_line)?;
            jsr_w(target_line)
        }
        0x8a => l2d,
        0x89 => l2f,
        0x88 => l2i,
        0x61 => ladd,
        0x2f => laload,
        0x7f => land,
        0x50 => lastore,
        0x94 => lcmp,
        0x09 => lconst_0,
        0x0a => lconst_1,
        0x12 => {
            let index = pop_u1_as_index(bytes)?;
            ldc(index)
        }
        0x13 => {
            let index = pop_u2_as_index(bytes)?;
            ldc_w(index)
        }
        0x14 => {
            let index = pop_u2_as_index(bytes)?;
            ldc2_w(index)
        }
        0x6d => ldiv,
        0x16 => {
            let index = pop_u1_as_index(bytes)?;
            lload(index)
        }
        0x1e => lload_0,
        0x1f => lload_1,
        0x20 => lload_2,
        0x21 => lload_3,
        0x69 => lmul,
        0x75 => lneg,
        0xab => {
            let lookup_switch = parse_lookupswitch(bytes, current_line)?;
            lookupswitch(lookup_switch)
        }
        0x81 => lor,
        0x71 => lrem,
        0xad => lreturn,
        0x79 => lshl,
        0x7b => lshr,
        0x37 => {
            let index = pop_u1_as_index(bytes)?;
            lstore(index)
        }
        0x3f => lstore_0,
        0x40 => lstore_1,
        0x41 => lstore_2,
        0x42 => lstore_3,
        0x65 => lsub,
        0x7d => lushr,
        0x83 => lxor,
        0xc2 => monitorenter,
        0xc3 => monitorexit,
        0xc5 => {
            let index = pop_u2_as_index(bytes)?;
            let dimensions = pop_u1_as_index(bytes)?;
            if dimensions == 0 {
                return Err(ParseError::InvalidOpCode);
            }
            multinewarray(index, dimensions)
        }
        0xbb => {
            let index = pop_u2_as_index(bytes)?;
            new(index)
        }
        0xbc => {
            let array_type = pop1(bytes).and_then(ArrayType::try_from)?;
            newarray(array_type)
        }
        0x00 => nop,
        0x57 => pop,
        0x58 => pop2,
        0xb5 => {
            let index = pop_u2_as_index(bytes)?;
            putfield(index)
        }
        0xb3 => {
            let index = pop_u2_as_index(bytes)?;
            putfield(index)
        }
        0xa9 => {
            let index = pop_u1_as_index(bytes)?;
            ret(index)
        }
        0xb1 => retrn,
        0x35 => saload,
        0x56 => sastore,
        0x11 => {
            let short_bits = utils::pop2(bytes)?;
            let short = i16::from_be_bytes(short_bits).into();
            sipush(short)
        }
        0x5f => swap,
        0xaa => {
            let table_switch = parse_tableswitch(bytes, current_line)?;
            tableswitch(table_switch)
        }
        0xc4 => {
            let w = parse_wide(bytes)?;
            wide(w)
        }
        _ => {
            return Err(ParseError::InvalidOpCode);
        }
    };

    Ok(op_code)
}

pub fn parse_n_opcodes<I>(
    bytes: &mut I,
    code_length: usize,
) -> Result<(Vec<OpCode>, HashMap<usize, usize>), ParseError>
where
    I: Iterator<Item = FileByte>,
{
    let byte_count = RefCell::new(0);

    let mut bytes = bytes
        .take(code_length)
        .scan(&byte_count, |byte_count, byte| {
            *byte_count.borrow_mut() += 1;
            Some(byte)
        })
        .peekable();

    let mut current_opcode_line = 0;

    let mut opcodes = Vec::new();

    // TODO: bench if array and do a binary search is not faster
    // pushing the lines one by one will have the array already sorted
    let mut jump_table = HashMap::new();

    while bytes.peek().is_some() {
        let opcode = parse_opcode(&mut bytes, current_opcode_line)?;
        let total_bytes_taken = *byte_count.borrow();
        let opcode_size = total_bytes_taken - current_opcode_line;
        jump_table.insert(current_opcode_line, opcodes.len());
        current_opcode_line += opcode_size;
        opcodes.push(opcode);
    }

    correct_jump_instructions(&mut opcodes, &jump_table)?;

    Ok((opcodes, jump_table))
}

pub fn update_jump(
    line: &mut usize,
    jump_table: &HashMap<usize, usize>,
    opcode: &'static str,
) -> Result<(), ParseError> {
    let vec_index = jump_table
        .get(line)
        .ok_or(ParseError::InvalidOpcodeJumpIndex {
            opcode,
            jump_target: *line,
        })?;
    *line = *vec_index;
    Ok(())
}

fn correct_jump_instructions<'a, I>(
    opcodes: I,
    jump_table: &HashMap<usize, usize>,
) -> Result<(), ParseError>
where
    I: IntoIterator<Item = &'a mut OpCode>,
{
    for opcode in opcodes {
        match opcode {
            OpCode::goto(line) => update_jump(line, jump_table, "goto")?,
            OpCode::goto_w(line) => update_jump(line, jump_table, "goto_w")?,
            OpCode::if_acmpeq(line) => update_jump(line, jump_table, "if_acmpeq")?,
            OpCode::if_acmpne(line) => update_jump(line, jump_table, "if_acmpne")?,
            OpCode::if_icmpeq(line) => update_jump(line, jump_table, "if_icmpeq")?,
            OpCode::if_icmpne(line) => update_jump(line, jump_table, "if_icmpne")?,
            OpCode::if_icmplt(line) => update_jump(line, jump_table, "if_icmplt")?,
            OpCode::if_icmpge(line) => update_jump(line, jump_table, "if_icmpge")?,
            OpCode::if_icmpgt(line) => update_jump(line, jump_table, "if_icmpgt")?,
            OpCode::if_icmple(line) => update_jump(line, jump_table, "if_icmple")?,
            OpCode::ifeq(line) => update_jump(line, jump_table, "ifeq")?,
            OpCode::ifne(line) => update_jump(line, jump_table, "ifne")?,
            OpCode::iflt(line) => update_jump(line, jump_table, "iflt")?,
            OpCode::ifge(line) => update_jump(line, jump_table, "ifge")?,
            OpCode::ifgt(line) => update_jump(line, jump_table, "ifgt")?,
            OpCode::ifle(line) => update_jump(line, jump_table, "ifle")?,
            OpCode::ifnonnull(line) => update_jump(line, jump_table, "ifnonnull")?,
            OpCode::ifnull(line) => update_jump(line, jump_table, "ifnull")?,
            OpCode::jsr_w(line) => update_jump(line, jump_table, "jsr_w")?,
            OpCode::lookupswitch(lus) => correct_lookupswitch_jumps(lus, jump_table)?,
            OpCode::tableswitch(ts) => correct_tableswitch_jumps(ts, jump_table)?,
            _ => {}
        }
    }

    Ok(())
}

// lookupswitch

#[derive(Debug, Clone)]
pub struct LookupSwitch {
    default: usize,
    pairs: Vec<LookupSwitchPair>,
}

impl LookupSwitch {
    pub fn find_jump(&self, value: i32) -> usize {
        // could do a binary search, but probably will not be that much efficient and doesn't look as good
        // [T]::binary_search return an index, and I would really preferred a reference

        self.pairs
            .iter()
            .find_map(|pair| (pair.value == value).then_some(pair.jump))
            .unwrap_or(self.default)
    }
}

#[derive(Debug, Clone)]
pub struct LookupSwitchPair {
    value: i32,
    jump: usize,
}

fn parse_lookupswitch<I>(bytes: &mut I, current_line: usize) -> Result<LookupSwitch, ParseError>
where
    I: Iterator<Item = FileByte>,
{
    let padding = (4 - ((current_line + 1) % 4)) % 4;

    skip_n(bytes, padding)?;
    let default = parse_u4_index_offset(bytes, current_line)?;
    let npairs = pop_u4_as_index(bytes)?;

    let mut pairs = Vec::with_capacity(npairs);

    for _ in 0..npairs {
        let pair = parse_lookupswitch_pair(bytes, current_line)?;
        pairs.push(pair);
    }

    Ok(LookupSwitch { default, pairs })
}

fn parse_lookupswitch_pair<I>(
    bytes: &mut I,
    current_line: usize,
) -> Result<LookupSwitchPair, ParseError>
where
    I: Iterator<Item = FileByte>,
{
    let value_bits = pop4(bytes)?;
    let value = i32::from_be_bytes(value_bits);
    let jump = parse_u4_index_offset(bytes, current_line)?;

    Ok(LookupSwitchPair { value, jump })
}

fn correct_lookupswitch_jumps(
    lus: &mut LookupSwitch,
    jump_table: &HashMap<usize, usize>,
) -> Result<(), ParseError> {
    let LookupSwitch { default, pairs } = lus;
    update_jump(default, jump_table, "lookupswitch_default")?;

    for pair in pairs {
        update_jump(&mut pair.jump, jump_table, "lookupswitch_pair")?;
    }

    Ok(())
}

// tableswitch
#[derive(Debug, Clone)]
pub struct TableSwitch {
    default: usize,
    offset: i32,
    jumps: Vec<usize>,
}

impl TableSwitch {
    fn get_jump_index(&self, index: i32) -> Option<usize> {
        if index < 0 {
            None
        } else {
            self.jumps.get(index as usize).copied()
        }
    }
    pub fn find_jump(&self, index: i32) -> usize {
        let index = index - self.offset;
        self.get_jump_index(index).unwrap_or(self.default)
    }
}

fn parse_tableswitch<I>(bytes: &mut I, current_line: usize) -> Result<TableSwitch, ParseError>
where
    I: Iterator<Item = FileByte>,
{
    let padding = 4 - ((current_line + 1) % 4);

    skip_n(bytes, padding)?;
    let default = parse_u4_index_offset(bytes, current_line)?;
    let low_bits = pop4(bytes)?;
    let low = i32::from_be_bytes(low_bits);
    let high_bits = pop4(bytes)?;
    let high = i32::from_be_bytes(high_bits);

    if low > high {
        return Err(ParseError::InvalidTableSwitchBounds);
    }

    let jumps_count = (high - low + 1) as usize; // always positive, low <= high

    let mut jumps = Vec::with_capacity(jumps_count);

    for _ in 0..jumps_count {
        let target_line = parse_u4_index_offset(bytes, current_line)?;
        jumps.push(target_line);
    }

    Ok(TableSwitch {
        default,
        offset: high - low,
        jumps,
    })
}

fn correct_tableswitch_jumps(
    ts: &mut TableSwitch,
    jump_table: &HashMap<usize, usize>,
) -> Result<(), ParseError> {
    let TableSwitch { default, jumps, .. } = ts;

    update_jump(default, jump_table, "tableswitch_default")?;

    for target_line in jumps {
        update_jump(target_line, jump_table, "tableswitch_target")?;
    }

    Ok(())
}

// wide

#[derive(Debug, Clone)]
#[allow(non_camel_case_types)]
pub enum Wide {
    iload(usize),
    fload(usize),
    aload(usize),
    lload(usize),
    dload(usize),
    istore(usize),
    fstore(usize),
    astore(usize),
    lstore(usize),
    dstore(usize),
    ret(usize),
    iinc(usize, i32),
}

fn parse_wide<I>(bytes: &mut I) -> Result<Wide, ParseError>
where
    I: Iterator<Item = FileByte>,
{
    use Wide::*;
    let tag = pop1(bytes)?;
    let index = pop_u2_as_index(bytes)?;
    match tag {
        0x19 => Ok(aload(index)),  // aload
        0x18 => Ok(dload(index)),  // dload
        0x17 => Ok(fload(index)),  // fload
        0x15 => Ok(iload(index)),  // iload
        0x16 => Ok(lload(index)),  // lload
        0x3a => Ok(astore(index)), // astore
        0x39 => Ok(dstore(index)), // dstore
        0x38 => Ok(fstore(index)), // fstore
        0x36 => Ok(istore(index)), // istore
        0x37 => Ok(lstore(index)), // lstore
        0xa9 => Ok(ret(index)),    // ret
        0x84 => {
            let delta_bits = utils::pop2(bytes)?;
            let delta = i16::from_be_bytes(delta_bits).into();
            Ok(iinc(index, delta))
        }
        _ => Err(ParseError::InvalidWideOpCode),
    }
}
