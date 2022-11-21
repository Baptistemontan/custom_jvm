use crate::runtime_types::{Code, ExceptionTable, OpCode};

pub fn basic_add_function() -> Code {
    let max_stack = 2;
    let max_locals = 2;
    let args_count = 2;
    let exception_table = ExceptionTable::new(None);

    use OpCode::*;
    let opcodes = vec![load_0, load_1, add, return_v];

    Code::new(max_stack, max_locals, opcodes, args_count, exception_table)
}

/*
    stack=2, locals=5, args_size=1
    0: iconst_0
    1: istore_1
    2: iconst_1
    3: istore_2
    4: iconst_0
    5: istore_3
    6: iload_3
    7: iload_0
    8: if_icmpge     27
    11: iload_2
    12: iload_1
    13: iadd
    14: istore        4
    16: iload_2
    17: istore_1
    18: iload         4
    20: istore_2
    21: iinc          3, 1
    24: goto          6
    27: iload_1
    28: ireturn
*/

pub fn fibonacci_calculator() -> Code {
    let max_stack = 2;
    let max_locals = 5;
    let args_count = 1;
    let exception_table = ExceptionTable::new(None);

    use OpCode::*;
    let opcodes = vec![
        iconst_0,
        store_1,
        iconst_1,
        store_2,
        iconst_0,
        store_3,
        load_3,
        load_0,
        if_icmpge(19),
        load_2,
        load_1,
        add,
        store_i { local_index: 4 },
        load_2,
        store_1,
        load_i { local_index: 4 },
        store_2,
        iinc {
            local_index: 3,
            delta: 1,
        },
        goto(6),
        load_1, // 19
        return_v,
    ];

    Code::new(max_stack, max_locals, opcodes, args_count, exception_table)
}
