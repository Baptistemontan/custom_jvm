use crate::parser::utils::{self, pop1, pop_u1_as_index, pop_u2_as_index, FileByte, ParseError};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ArrayType {
    Boolean,
    Char,
    Float,
    Double,
    Byte,
    Short,
    Int,
    Long,
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

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[allow(non_camel_case_types)]
pub enum OpCode {
    aaload,                      // 0x32
    aastore,                     // 0x53
    aconst_null,                 // 0x01
    aload(usize),                // 0x19
    aload_0,                     // 0x2a
    aload_1,                     // 0x2b
    aload_2,                     // 0x2c
    aload_3,                     // 0x2d
    anewarray(usize),            // 0xbd
    areturn,                     // 0xb0
    arraylength,                 // 0xbe
    astore(usize),               // 0x3a
    astore_0,                    // 0x4b
    astore_1,                    // 0x4c
    astore_2,                    // 0x4d
    astore_3,                    // 0x4e
    athrow,                      // 0xbf
    baload,                      // 0x33
    bastore,                     // 0x54
    bipush(i32),                 // 0x10
    caload,                      // 0x34
    castore,                     // 0x55
    checkcast(usize),            // 0xc0
    d2f,                         // 0x90
    d2i,                         // 0x8e
    d2l,                         // 0x8f
    dadd,                        // 0x63
    daload,                      // 0x31
    dastore,                     // 0x52
    dcmpg,                       // 0x98
    dcmpl,                       // 0x97
    dconst_0,                    // 0x0e
    dconst_1,                    // 0x0f
    ddiv,                        // 0x6f
    dload(usize),                // 0x18
    dload_0,                     // 0x26
    dload_1,                     // 0x27
    dload_2,                     // 0x28
    dload_3,                     // 0x29
    dmul,                        // 0x6b
    dneg,                        // 0x77
    drem,                        // 0x73
    dreturn,                     // 0xaf
    dstore(usize),               // 0x39
    dstore_0,                    // 0x47
    dstore_1,                    // 0x48
    dstore_2,                    // 0x49
    dstore_3,                    // 0x4a
    dsub,                        // 0x67
    dup,                         // 0x59
    dup_x1,                      // 0x5a
    dup_x2,                      // 0x5b
    dup2,                        // 0x5c
    dup2_x1,                     // 0x5d
    dup2_x2,                     // 0x5e
    f2d,                         // 0x8d
    f2i,                         // 0x8b
    f2l,                         // 0x8c
    fadd,                        // 0x62
    faload,                      // 0x30
    fastore,                     // 0x51
    fcmpg,                       // 0x96
    fcmpl,                       // 0x95
    fconst_0,                    // 0x0b
    fconst_1,                    // 0x0c
    fconst_2,                    // 0x0d
    fdiv,                        // 0x6e
    fload(usize),                // 0x17
    fload_0,                     // 0x22
    fload_1,                     // 0x23
    fload_2,                     // 0x24
    fload_3,                     // 0x25
    fmul,                        // 0x6a
    fneg,                        // 0x76
    frem,                        // 0x72
    freturn,                     // 0xae
    fstore(usize),               // 0x38
    fstore_0,                    // 0x43
    fstore_1,                    // 0x44
    fstore_2,                    // 0x45
    fstore_3,                    // 0x46
    fsub,                        // 0x66
    getfield(usize),             // 0xb4
    getstatic(usize),            // 0xb2
    goto(usize),                 // 0xa7 | TODO
    goto_w(usize),               // 0xc8 | TODO
    i2b,                         // 0x91
    i2c,                         // 0x92
    i2d,                         // 0x87
    i2f,                         // 0x86
    i2l,                         // 0x85
    i2s,                         // 0x93
    iadd,                        // 0x60
    iaload,                      // 0x2e
    iand,                        // 0x7e
    iastore,                     // 0x4f
    iconst_m1,                   // 0x02
    iconst_0,                    // 0x03
    iconst_1,                    // 0x04
    iconst_2,                    // 0x05
    iconst_3,                    // 0x06
    iconst_4,                    // 0x07
    iconst_5,                    // 0x08
    idiv,                        // 0x6c
    if_acmpeq(usize),            // 0xa5 | TODO
    if_acmpne(usize),            // 0xa6 | TODO
    if_icmpeq(usize),            // 0x9f | TODO
    if_icmpne(usize),            // 0xa0 | TODO
    if_icmplt(usize),            // 0xa1 | TODO
    if_icmpge(usize),            // 0xa2 | TODO
    if_icmpgt(usize),            // 0xa3 | TODO
    if_icmple(usize),            // 0xa4 | TODO
    ifeq(usize),                 // 0x99 | TODO
    ifne(usize),                 // 0x9a | TODO
    iflt(usize),                 // 0x9b | TODO
    ifge(usize),                 // 0x9c | TODO
    ifgt(usize),                 // 0x9d | TODO
    ifle(usize),                 // 0x9e | TODO
    ifnonnull(usize),            // 0xc7 | TODO
    ifnull(usize),               // 0xc6 | TODO
    iinc(usize, i32),            // 0x84
    iload(usize),                // 0x15
    iload_0,                     // 0x1a
    iload_1,                     // 0x1b
    iload_2,                     // 0x1c
    iload_3,                     // 0x1d
    imul,                        // 0x68
    ineg,                        // 0x74
    instanceof(usize),           // 0xc1
    invokedynamic(usize),        // 0xba
    invokeinterface(usize, u8),  // 0xb9
    invokespecial(usize),        // 0xb7
    invokestatic(usize),         // 0xb8
    invokevirtual(usize),        // 0xb6
    ior,                         // 0x80
    irem,                        // 0x70
    ireturn,                     // 0xac
    ishl,                        // 0x78
    ishr,                        // 0x7a
    istore(usize),               // 0x36
    istore_0,                    // 0x3b
    istore_1,                    // 0x3c
    istore_2,                    // 0x3d
    istore_3,                    // 0x3e
    isub,                        // 0x64
    iushr,                       // 0x7c
    ixor,                        // 0x82
    jsr(usize),                  // 0xa8 | TODO
    jsr_w(usize),                // 0xc9 | TODO
    l2d,                         // 0x8a
    l2f,                         // 0x89
    l2i,                         // 0x88
    ladd,                        // 0x61
    laload,                      // 0x2f
    land,                        // 0x7f
    lastore,                     // 0x50
    lcmp,                        // 0x94
    lconst_0,                    // 0x09
    lconst_1,                    // 0x0a
    ldc(usize),                  // 0x12
    ldc_w(usize),                // 0x13
    ldc2_w(usize),               // 0x14
    ldiv,                        // 0x6d
    lload(usize),                // 0x16
    lload_0,                     // 0x1e
    lload_1,                     // 0x1f
    lload_2,                     // 0x20
    lload_3,                     // 0x21
    lmul,                        // 0x69
    lneg,                        // 0x75
    lookupswitch,                // 0xab | TODO
    lor,                         // 0x81
    lrem,                        // 0x71
    lreturn,                     // 0xad
    lshl,                        // 0x79
    lshr,                        // 0x7b
    lstore(usize),               // 0x37
    lstore_0,                    // 0x3f
    lstore_1,                    // 0x40
    lstore_2,                    // 0x41
    lstore_3,                    // 0x42
    lsub,                        // 0x65
    lushr,                       // 0x7d
    lxor,                        // 0x83
    monitorenter,                // 0xc2
    monitorexit,                 // 0xc3
    multinewarray(usize, usize), // 0xc5
    new(usize),                  // 0xbb
    newarray(ArrayType),         // 0xbc
    nop,                         // 0x00
    pop,                         // 0x57
    pop2,                        // 0x58
    putfield(usize),             // 0xb5
    putstatic(usize),            // 0xb3
    ret(usize),                  // 0xa9 | TODO
    retrn,                       // 0xb1 | return
    saload,                      // 0x35
    sastore,                     // 0x56
    sipush(i32),                 // 0x11
    swap,                        // 0x5f
    tableswitch,                 // 0xaa | TODO
    wide,                        // 0xc4 | TODO
}

fn parse_opcode<I>(bytes: &mut I) -> Result<OpCode, ParseError>
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
        0xa7 => todo!(), // goto
        0xc8 => todo!(), // goto_w
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
        0xa5 => todo!(), // if_acmpeq
        0xa6 => todo!(), // if_acmpne
        0x9f => todo!(), // if_icmpeq
        0xa0 => todo!(), // if_icmpne
        0xa1 => todo!(), // if_icmplt
        0xa2 => todo!(), // if_icmpge
        0xa3 => todo!(), // if_icmpgt
        0xa4 => todo!(), // if_icmple
        0x99 => todo!(), // ifeq
        0x9a => todo!(), // ifne
        0x9b => todo!(), // iflt
        0x9c => todo!(), // ifge
        0x9d => todo!(), // ifgt
        0x9e => todo!(), // ifle
        0xc7 => todo!(), // ifnonnull
        0xc6 => todo!(), // ifnull
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
            invokeinterface(index, count)
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
        0xa8 => todo!(), // jsr
        0xc9 => todo!(), // jsr_w
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
        0xab => todo!(), // lookupswitch
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
        0xa9 => todo!(), // ret
        0xb1 => retrn,
        0x35 => saload,
        0x56 => sastore,
        0x11 => {
            let short_bits = utils::pop2(bytes)?;
            let short = i16::from_be_bytes(short_bits).into();
            sipush(short)
        }
        0x5f => swap,
        0xaa => todo!(), // tableswitch
        0xc4 => todo!(), // wide
        _ => unimplemented!(),
    };

    Ok(op_code)
}

pub fn parse_n_opcodes<I>(bytes: &mut I, code_length: usize) -> Result<Vec<OpCode>, ParseError>
where
    I: Iterator<Item = FileByte>,
{
    let mut bytes = bytes.take(code_length).peekable();

    let mut opcodes = Vec::new();

    while bytes.peek().is_some() {
        let opcode = parse_opcode(&mut bytes)?;
        opcodes.push(opcode);
    }

    Ok(opcodes)
}
