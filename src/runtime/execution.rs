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

type SymRef = usize;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[allow(non_camel_case_types)]
pub enum OpCode {
    aaload,
    aastore,
    aconst_null,
    aload(usize),
    aload_0,
    aload_1,
    aload_2,
    aload_3,
    anewarray(usize),
    areturn,
    arraylength,
    astore(usize),
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
    checkcast(usize),
    d2f,
    d2i,
    d2l,
    dadd,
    daload,
    dastore,
    dcmpg,
    dcmpl,
    dconst_0,         // 0xe
    dconst_1,         // 0xf
    ddiv,             // 0x6f
    dload(usize),     // 0x18
    dload_0,          // 0x26
    dload_1,          // 0x27
    dload_2,          // 0x28
    dload_3,          // 0x29
    dmul,             // 0x6b
    dneg,             // 0x77
    drem,             // 0x73
    dreturn,          // 0xaf
    dstore(usize),    // 0x39
    dstore_0,         // 0x47
    dstore_1,         // 0x48
    dstore_2,         // 0x49
    dstore_3,         // 0x4a
    dsub,             // 0x67
    dup,              // 0x59
    dup_x1,           // 0x5a
    dup_x2,           // 0x5b
    dup2,             // 0x5c
    dup2_x1,          // 0x5d
    dup2_x2,          // 0x5e
    f2d,              // 0x8d
    f2i,              // 0x8b
    f2l,              // 0x8c
    fadd,             // 0x62
    faload,           // 0x30
    fastore,          // 0x51
    fcmpg,            // 0x96
    fcmpl,            // 0x95
    fconst_0,         // 0xb
    fconst_1,         // 0xc
    fconst_2,         // 0xd
    fdiv,             // 0x6e
    fload(usize),     // 0x17
    fload_0,          // 0x22
    fload_1,          // 0x23
    fload_2,          // 0x24
    fload_3,          // 0x25
    fmul,             // 0x6a
    fneg,             // 0x76
    frem,             // 0x72
    freturn,          // 0xae
    fstore(usize),    // 0x38
    fstore_0,         // 0x43
    fstore_1,         // 0x44
    fstore_2,         // 0x45
    fstore_3,         // 0x46
    fsub,             // 0x66
    getfield(usize),  // 0xb4
    getstatic(usize), // 0xb2
    goto(usize),      // 0xa7
    goto_w(usize),    // 0xc8
    i2b,              //  0x91
    i2c,              // 0x92
    i2d,              // 0x87
    i2f,              // 0x86
    i2l,              // 0x85
    i2s,              // 0x93
    iadd,             // 0x60
    iaload,           // 0x2e
    iand,             // 0x7e
    iastore,          // 0x4f
    iconst_m1,        // 0x2
    iconst_0,         // 0x3
    iconst_1,         // 0x4
    iconst_2,         // 0x5
    iconst_3,         // 0x6
    iconst_4,         // 0x7
    iconst_5,         // 0x8
    idiv,             // 0x6c
    // ref comparaison
    if_acmpeq(usize), // 0xa5
    if_acmpne(usize), // 0xa6
    // int comparaison
    if_icmpeq(usize), // 0x9f
    if_icmpne(usize), // 0xa0
    if_icmplt(usize), // 0xa1
    if_icmpge(usize), // 0xa2
    if_icmpgt(usize), // 0xa3
    if_icmple(usize), // 0xa4
    // int comp with 0
    ifeq(usize), // 0x99
    ifne(usize), // 0x9a
    iflt(usize), // 0x9b
    ifge(usize), // 0x9c
    ifgt(usize), // 0x9d
    ifle(usize), // 0x9e
    // check ref nullity
    ifnonnull(usize), // 0xc7
    ifnull(usize),    // 0xc6

    iinc(usize, i32), // 0x84
    iload(usize),     // 0x15
    iload_0,          // 0x1a
    iload_1,          // 0x1b
    iload_2,          // 0x1c
    iload_3,          // 0x1d
    imul,             // 0x68
    ineg,             // 0x74
    // function call
    instanceof(usize),           // 0xc1
    invokedynamic(usize),        // 0xba
    invokeinterface(usize),      // 0xb9
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
    jsr(usize),                  // 0xa8
    jsr_w(usize),                // 0xc9
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
    lookupswitch,                // TODO | 0xab
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
    new(SymRef),                 // 0xbb
    newarray(ArrayType),         // 0xbc
    nop,                         // 0x00
    pop,                         // 0x57
    pop2,                        // 0x58
    putfield(SymRef),            // 0xb5
    putstatic(SymRef),           // 0xb3
    ret(usize),                  // 0xa9
    retur,                       // return | 0xb1
    saload,                      // 0x35
    sastore,                     // 0x56
    sipush(i32),                 // 0x11
    swap,                        // 0x5f
    tableswitch,                 // TODO | 0xaa
    wide,                        // TODO | 0xc4
}
