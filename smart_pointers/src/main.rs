// Smart Pointers
//  - Box Smart Pointers
//  - use case of box smart pointers
//fn main() {
/* let single_value = Box::new(0.625);
    let x = 0.625;
    println!("Are the values equal {}", x == *single_value);

    let mut stack_var = 4;
    let stack_ref = &stack_var;

let heap_var = Box::new(stack_var);

stack_var = 5;
println!("The value of stack_var = {}, and the heap_var = {}", stack_var, heap_var);

let point = Box::new((100,125));
println!("{} {}", 100 == point.0, point.1);

let x = *point; */
/* 
#[derive(Debug)]
enum List {
    Cons(i32, Option<Box<List>>),
}

use List::{Cons, Nil};

fn main() {
    let list = Cons(1, Some(Box::new(Cons(2, Some(Box::new(Cons(3, None)))))));
    println!("{:?}", list);
} */

struct MySmartPointer{value: i32}

impl MySmartPointer {
    fn new(x: i32) -> MySmartPointer {
        MySmartPointer {
            value: x
        }
    }
}

use std::ops::Deref;

impl Deref for MySmartPointer {
    type Target = i32;
    fn deref(&self) -> &i32 {
        &self.value
    }
    
}

impl Drop for MySmartPointer {
    fn drop(&mut self) {
        println!("dropping my smart pointer from memory {:?}", self.value)
    }
}

fn main() {
    let a = 50;
    let b = Box::new(a);
    println!("{}", 50 == a);
    println!("{}", 50 == *b);
    //println!("{}", a == b);
    let sptr1 = MySmartPointer::new(a);
    let sprt2 = MySmartPointer::new(a+3);
    let sptr3 = MySmartPointer::new(a+6);
    let sprt4 = MySmartPointer::new(*b);

    println!("{}", a == *sptr1);

    //drop(spr1);
    
}
