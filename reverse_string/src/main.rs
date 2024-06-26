fn new_stack(maxsize: usize) -> Vec<char> {
    let vec: Vec<char> = Vec::with_capacity(maxsize);
    vec
}

fn pop(stack: &mut Vec<char>) -> Option<char> {
    let popped = stack.pop();
    popped
}

fn push(stack: &mut Vec<char>, item: char, maxsize: usize) {
    if stack.len() == maxsize {
        println!("Can not add more");
    } else {
        stack.push(item);
    }
}

fn size(stack: &Vec<char>) -> usize {
    stack.len()
}

fn input() -> char {
    let mut c: String = String::new();
    std::io::stdin()
    .read_line(&mut c)
    .expect("faild to read input");

    let c: char = c.trim().parse().expect("invaild input");
    c
}

fn main() {
    let input_string: String = String::from("Welcome to rust");
    let input_size = input_string.len();
    let mut stack = new_stack(input_size);
    let mut reversed_str = String::new();

    for i in input_string.chars() {
        push(&mut stack, i, input_size);
    }

    for i in 0..size(&stack) {
        reversed_str.push(pop(&mut stack).unwrap());
    }

    println!("The input string is {:?}", input_string);
    println!("The reversed string is {:?}", reversed_str);
    
}
