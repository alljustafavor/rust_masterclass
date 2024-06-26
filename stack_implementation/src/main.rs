fn new_stack(maxsize: usize) -> Vec<u32> {
    let vec: Vec<u32> = Vec::with_capacity(maxsize);
    vec
}

fn pop(stack: &mut Vec<u32>) -> Option<u32> {
    let poped_val = stack.pop();
    println!("The poped value {:?}", poped_val);
    poped_val
}

fn push(stack: &mut Vec<u32>, item: u32, maxsize: usize) {
    if stack.len() == maxsize {
        println!("Can not add more");
    } else {
        stack.push(item);
        println!("Stack: {:?}", stack);
    }
}

fn size(stack: &Vec<u32>) -> usize {
    stack.len()
}

fn input() -> u32 {
    let mut n: String = String::new();
    std::io::stdin()
    .read_line(&mut n)
    .expect("failed to read input");

    let n: u32 = n.trim().parse().expect("invaild input");
    n
}

fn main() {
    println!("Let us first create a stack for our use");
    println!("Enter the size for the stack");
    let size_stack = input();

    let mut stack: Vec<u32> = new_stack(size_stack as usize);

    loop {
        println!("\n\n ******* menu *******\n");
        println!("
            1. Push \n
            2. Pop \n
            3. Display \n
            4. Size \n
            5. Exit
            ");
        println!("\n Enter you choice: ");
        let choice: u32 = input();
        match choice {
            1 => {
                println!("Enter the value to be inserted: ");
                let item = input();
                push(&mut stack, item, size_stack as usize);
            }
            2 => println!("The element to be poped: {:?}", pop(&mut stack)),
            3 => println!("The elements: {:?}", stack),
            4 => println!("The size of the stack is {}", size(&stack)),
            5 => {
                println!("Exiting...");
                break
            },
            _ => println!("Invaild, Please try again."),
        };

        println!("Do you wish to continue 1 = \"yes\" 0 = \"No\"");
        let status = input();
        if status == 1 {
            continue;
        } else {
            break;
        }

    };

}
