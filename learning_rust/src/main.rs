fn main() {
    //-------------------------------------------------------------------------------------------------------
    //|                                     Program outputs & Comments                                      |
    //-------------------------------------------------------------------------------------------------------

    // this is our first program in this course
    // this is the second line of the comment

    /*
    This is a
    multiple line
    comment
    */

    println!("Hello from Rust!");

    //print!(10);

    println!("The value is {}", 10);

    println!("My first name is {} and my last is {}", "Joseph", "Noblett");

    print!("This is a print command");

    print!("This is going to be on the same line, ");
    print!("This is going to be on the same line, ");
    print!("This is going to be on the same line.");

    println!("");

    print!(
        "this is going to be 
    on multiple 
    lines."
    );

    println!("\n\n this is going to be outputted after one line");

    println!("\t this will have some a tab inserted at the beginning");
    
    println!(
        "this is some text which will be overwritten \r this text will only appear on the screen"
    );

    println!("This will print single quote \' and this double quotes\"");

    println!("This will print one backslash \\");

    println!(
        "\n doing {2} from {1} years and i {0} it",
        "like", 20, "programming"
    );

    println!(
        "{language} is a system programming language which is cool to {activity} in.",
        activity = "Code",
        language = "Rust"
    );

    println!("The sum of 25 + 10 = {}", 25 + 10);

    //-------------------------------------------------------------------------------------------------------
    //|                                     Variables & Data Types                                        |
    //-------------------------------------------------------------------------------------------------------

    let x: f32 = 15.0;
    // println!("The value of variable x = {}", x);

    println!("The maximum number in i8 is = {}", std::i8::MAX);

    let z = 3.65;

    // let a = x+z;
    println!("The maximum number in f32 is = {}", std::f32::MAX);

    let status: bool = false;
    println!("The value of some of out variable are {:?}", (z, x, status));
    
    let not_equals = 18 != 18;
    println!("The value of condition is 18 != 18 is {}", not_equals);

    //-------------------------------------------------------------------------------------------------------
    //|                                     Shadowing & Constants                                         |
    //-------------------------------------------------------------------------------------------------------

    let (first_number, second_number) = (250, 480.22);
    let large_number = 1_000_000;

    // let overflow_number:u8 = 256;
    let x = 255;
    println!(
        "The value of the variable in octal is {:o} and in hexadecimal is {:X} and in binary {:b}",
        x, x, x
    );

    let Number = 45;

    let n1 = 14;
    let n2 = 15.6;
    
    let n3 = n1 + n2 as i32;
    println!("The value of n3 = {}", n3);

    // Shadowing

    let s = 5;
    let s = 5 * 5;

    let p = 5;
    //let p: i32 = 5 * 5;

    let q = 32;
    let q = 'A';

    let r = 65;
    {
        //let r = 50;
        println!("Inside the code segment r: {}", r);
    }
    println!("Outside the code segment r: {}", r);

    const MAX_SALARY: u32 = 100_000;

    //-------------------------------------------------------------------------------------------------------
    //|                                     Compound Data Types - Strings                                   |
    //-------------------------------------------------------------------------------------------------------

    let some_string = "Fixed length string";

    let mut growable_string = String::from("This string will grow");
    println!("The string is: \"{}\" ", growable_string);

    growable_string.push('s');
    println!("The string is: \"{}\" ", growable_string);

    growable_string.pop();
    println!("The string is \"{}\" ", growable_string);

    growable_string.push_str("Which i will use ");
    println!(
        "Basic function on strings,
    is_emtpy(): {},
    length: {},
    bytes: {},
    contains 'use': {}",
        growable_string.is_empty(),
        growable_string.len(),
        growable_string.capacity(),
        growable_string.contains("use")
    );

    growable_string.push_str("       ");
    let str_len = growable_string.trim().len();

    let number = 6;
    let num_str = number.to_string();
    println!(
        "Is the number really a string {}",
        number.to_string() == "6"
    );
    
    let some_char = 'a';
    let char_str = some_char.to_string();

    let my_name = "Joseph Noblett".to_string();

    let empty_string = String::new();
    println!("Length is {}", empty_string.len());

    let s_1 = "Joseph".to_string();
    let s_2 = "Noblett".to_string();
    let s3 = format!("My first name is {} and my last is {}", s_1, s_2);

    let concat = format!("{}{}", s_1, s_2);

    //-------------------------------------------------------------------------------------------------------
    //|                                           Tuples & Arrays                                           |
    //-------------------------------------------------------------------------------------------------------

    let my_information = ("Salary", 40_000);
    println!("{} is equal to {}", my_information.0, my_information.1);

    println!(
        "Another way of printing the whole tuple is {:?}",
        my_information
    );

    // Destructuring

    let (salary, salary_value) = my_information;

    let salary = my_information.0;
    let salary_value = my_information.1;

    let nested_tuple = (4, 5.0, (3, 2), "Hello");
    let element = nested_tuple.2 .0;

    let empty_tuple = ();

    let mut number_array: [i32; 5] = [4, 5, 6, 8, 9];

    println!("{}", number_array[0]);

    println!("{:?}", number_array);

    number_array[4] = 5;
    let array_with_same_elements = [0; 10];

    let mut string_array_1 = ["apple", "tomato", "grapes"];
    let string_array_2 = ["Unknown"; 6];
    string_array_1[0] = "Austin Bun";

    let char_array = ['a', 'p', 'p', 'l', 'e'];

    let mut number_array_1 = [4,5,6,8,9];
    let ex_subset = &number_array_1[0..3];
    let in_subset: &[i32] = &number_array_1[0..=3];
    println!("The ex_subset of the array contains: {:?}", ex_subset);
    println!("The in_subset of the array contains: {:?}", in_subset);

    println!("Elements in the array are {}", number_array_1.len());

    println!("The array is occupying {} bytes", std::mem::size_of_val(&number_array_1));

    // number_array_1[10] = 5;

    let check_index = number_array_1.get(100);
    println!("{:?}", check_index);

    //-------------------------------------------------------------------------------------------------------
    //|                                           Vectors                                                   |
    //-------------------------------------------------------------------------------------------------------

    let mut number_vec = vec![4,5,6,8,9,10,11,12,15,16,12,10];
    println!("{}", number_vec[0]);
    println!("{:?}", number_vec);

    number_vec[4] = 5;
    println!("{:?}", number_vec);

    let array_with_same_elements = vec![0;10];

    let mut string_array_1 = vec!["apple", "tomato", "grapes"];
    string_array_1[0] = "Joseph";

    let char_vec = vec!['a', 'p', 'p', 'l', 'e'];

    let subset_vec = &&number_vec[0..3];
    println!("The subset of value of the array are {:?}", subset_vec);

    println!("Elements in the array are {}", number_vec.len());

    let check_index = number_vec.get(100);
    println!("{:?}",check_index);

    number_vec.push(30);
    number_vec.push(40);
    println!("The values are {:?}", number_vec);

    number_vec.remove(5);
    println!("The vectors values are {:?}", number_vec);

    println!("The value of 10 exist {}", number_vec.contains(&10));

    //-------------------------------------------------------------------------------------------------------
    //|                                         Functions  & Inputs                                         |
    //-------------------------------------------------------------------------------------------------------
    
    basic_fn();
    function_with_inputs("Joseph", 50000);
    let test_name = "Joseph";
    let test_salary = 50001;
    function_with_inputs( test_name, test_salary);
    let answer = function_with_inputs_outputs(10, 15);
    println!("The answer is {}", answer);
    
    let multiple_answers = function_with_inputs_multiple_outputs(10, 15);
    println!("multi: {}, add: {}, sub: {}", multiple_answers.0, multiple_answers.1, multiple_answers.2);

    let full_name = {
        let first_name = "Joseph";
        let last_name = "Noblett";
        format!("{} {}", first_name, last_name)
    };
    println!("My full name: {}", full_name);


    // let mut n = String::new();
    // std::io::stdin()
    // .read_line(&mut n)
    // .expect("failed to read input");

    // let n: f64 = n.trim().parse().expect("invalid input");
    // println!("{:?}", n);

    //-------------------------------------------------------------------------------------------------------
    //|                                         Rust Ownership                                              |
    //|                      - Each value in Rust has a variable that's called it owner                     |
    //|                      -There can be only one owner at a time                                         |
    //|                      - When the owner goes out of scope, the value will be dropped                  |
    //-------------------------------------------------------------------------------------------------------

    let x = 32.6;
    let y = x;

    println!("x: {}, y: {}", x, y);

    let s1 = String::from("abc");
    let s2 = &s1;
    println!("s1: {}, s2: {}", s1, s2);

    let vec_1 = vec![5,6,9,8,7];
    let vec_2 = vec_1.clone();
    println!("Vec 1: {:?}, Vec 2: {:?}", vec_1, vec_2);

    let stack_num = 32;
    let mut heap_vec = vec![4,5,6];

    stack_function(stack_num);
    println!("The value inside the main of stack_num: {}", stack_num);

    heap_function(&mut heap_vec);
    println!("The value inside the main heap_vec: {:?}", heap_vec);

    let large_data1 = String::from("This is the first long string");
    let large_data2 = String::from("This is the second long string");

    let huge_data = vec![&large_data1, &large_data2];

    //-------------------------------------------------------------------------------------------------------
    //|                                        Reference Rules                                              |
    //|                               - One mutable reference in a scope : helps prevent race conditions    |
    //|                               - Many immutable references                                           |
    //|                               - Mutable and immutable cannot coexist                                |
    //|                               - Scope of a reference                                                |
    //|                               - Data shouldn't change when immutable reference are in scope         |
    //-------------------------------------------------------------------------------------------------------

    // One mutable ref
    let mut heap_num = vec![4,5,6];
    let ref1 = &mut heap_num;
    let ref2 = &ref1; // &mut heap_num 
    println!("ref1: {:?}, ref2: {:?}", ref1, ref2);

    // Many immutable refs
    let heap_num = vec![4,5,6];
    let ref1 = &heap_num;
    let ref2 = &heap_num;
    println!("ref1: {:?}, ref2: {:?}", ref1, ref2);

    // Mutable and immutable cannot coexist
    // let mut heap_num = vec![4,5,6];
    // let ref1 = &heap_num;
    // let ref2 = &heap_num;
    // let ref3 = &mut heap_num;
    // println!("ref1: {:?}, ref2: {:?}, ref3: {:?}", ref1, ref2, ref3)

    let mut heap_num = vec![4,5,6];
    let ref1 = &heap_num;
    let ref2 = &heap_num;
    println!("ref1: {:?}, ref2: {:?}", ref1, ref2);
    let ref3 = &mut heap_num;

    let mut heap_num = vec![4,5,6];
    let ref1 = &heap_num;
    let ref2 = &heap_num;

    println!("ref1: {:?}, ref2: {:?}", ref1, ref2);
    heap_num.push(68);

    //-------------------------------------------------------------------------------------------------------
    //|                                              Dereferencing                                          |
    //-------------------------------------------------------------------------------------------------------
    
    let mut some_data = 42;
    let ref_1 = &mut some_data;
    let deref_copy = *ref_1;              // stack
    *ref_1 = 13;
    println!("some_data is: {some_data}, deref_copy is: {deref_copy}");

    let mut heap_data = vec![5,6,7];
    let ref_1 = &heap_data;
    let ref_2 = ref_1;
    let ref_3 = ref_1;
    let deref_copy = ref_1.clone();  // heap

    //__________________________
    let move_out = ref_1;
    //let move_out_again = ref_1;

    //-------------------------------------------------------------------------------------------------------
    //|                                         Control Structures                                          |
    //-------------------------------------------------------------------------------------------------------

    // Simple if -> General Syntax

    let some_number = 40;
    if some_number < 50 {
        println!("The number is less than 50");
    }

    let marks = 65;
    if marks >= 60 && marks <= 70 {
        println!("grade: D");
    }

    let input1 = true;
    let input2 = false;

    if input1 == true || input2 == true {
        println!("Or circuit")
    }

    let input_1 = true;
    if input_1 != false {
        println!("This will execute when the input is not false")
    }

    let input_1 = true;
    let input_2 = false;
    let number = 60;

    if (input_1 == true && input_2 ==false) || number < 50 {
        println!("This part will execute based on the conditions above");
    }

    let marks = 80;
    if marks > 50 {
        println!("You passed!");
    } else {
        println!("You failed!");
    }

    let marks = 95;
    let mut grade = 'N';
    if marks >= 90 {
        grade = 'A';
    } else if marks >= 80 {
        grade = 'B';
    } else if marks >= 70 {
        grade = 'C';
    } else if marks >= 60 {
        grade = 'D'
    } else {
        grade = 'F'
    }
    println!("Your grade: {}", grade);

    println!("Enter a number");
    let mut some_num = String::new();

    std::io::stdio()
    .read_line(&mut some_num)
    .expect("failed to read input")
    

}

fn heap_function(var: &mut Vec<i32>) {
    var.push(50);
    println!("Var: {:?}", var);
}

fn stack_function(stack_num:i32) {
    println!("stack_num: {}", stack_num);
}

fn basic_fn() {
    println!("This is a basic function");
}

fn function_with_inputs(name: &str, salary: i32) {
   println!("The name is {} and the salary is {}", name, salary);
}

fn function_with_inputs_outputs(num1: i32, num2: i32) -> i32 {
    num1 * num2
}

fn function_with_inputs_multiple_outputs(num1: i32, num2: i32) -> (i32, i32, i32) {
    ((num1 * num2), (num1 + num2), (num1 - num2))
}
