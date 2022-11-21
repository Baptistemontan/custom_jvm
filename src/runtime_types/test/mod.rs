use super::{Stack, Object};

mod code_creation;

use code_creation::*;

#[test]
fn test_basic_add() {
    let code = basic_add_function();
    let mut stack = Stack::new(2);
    let a = 3;
    let b = 8;

    stack.push(Object::Int(a));
    stack.push(Object::Int(b));
    let result = code.execute(&mut stack);

    assert_eq!(result, Ok(Ok(Some(Object::Int(a + b)))));
}

#[test]
fn test_fibonacci() {
    fn fib(n: i32) -> i32 {
        let mut a = 0;
        let mut b = 0;
        for _ in 0..n {
            let tmp = a + b;
            a = b;
            b = tmp;
        }
        a
    }
    
    let code = fibonacci_calculator();
    let mut stack = Stack::new(1);
    let n = 30;
    stack.push(Object::Int(n));
    let result = code.execute(&mut stack);
    let should_be = fib(n);

    assert_eq!(result, Ok(Ok(Some(Object::Int(should_be)))))
}