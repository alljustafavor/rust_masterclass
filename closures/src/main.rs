/* fn main() {
    /* let x = 5;
    let square = |num: i32| println!("The square of the variable is {}", num * num);
    let square = |num: i32| println!("The cube of the variable is {}", num * num * num);
    square(x);

    let y = 15;
    square(y);
}  */
    let print_info = |general_info: String, name: &str, age| println!("{}\n\t {}: {}", general_info, name, age);
    let general_info = String::from("The details are ");
    let (person_name, person_age) = (String::from("Joseph"), 25);
    print_info(general_info, &person_name, &person_age);

    println!("The variable has been moved {}", person_name);
} */

/* fn main() {
    let square = |num| num * num;
    let x = 5;
    square(5);
    
} */

/* fn division<F: Fn(f32) -> bool>(x: f32, y: f32, f: F) {
   if f(y) == true {
        println!("The division result is {}", x/y);
    } else {
        println!("The division is not possible");
    } 
}

fn main() {
    let division_status = |y: f32| {if y != 0.0 {true} else {false}};

    division(5.0, 10.0, division_status);
    division(54.0, 0.0, division_status);
    
} */
/* 
fn main() {
  /*   let some_closure_1 = |x: u32| -> u32 {x+1};
    let some_closure_2 = |x| {x+1};
    let some_closure_3 = |x|x+1;
 */
    let mut vec_1 = vec![1,2,3];
    let mut some_closure = || {
        vec_1.push(35);
    };
    some_closure();
    println!("Vec 1: {:?}", vec_1);
    
} */

//////////////////////////////
///     Function Types     /// 
//////////////////////////////     

/* fn max(x: i32, y: i32) -> i32 {
    if x > y {x} else {y}
}

fn min(x: i32, y: i32) -> i32 {
    if x < y {x} else {y}
}

fn main() {
    let mut f = max;
    println!("The maximum of the two values is {}", f(2,3));
} */

/* fn prints_name(name: &str) {
    println!("The name is {}", name);
}

fn prints_full_info(f: fn(&str), some_one: &str, age: i32) {
    f(some_one);
    println!(" and my age is {}", age);
}

fn main() {
    let (my_name, my_age) = (String::from("Joseph"), 25);
    prints_full_info(prints_name, &my_name, my_age);
    
} */

fn add_one(x: i32) -> i32 {
    x+1
}

fn do_twice(f: fn(i32) -> i32, arg: i32) -> i32 {
    f(arg) + f(arg)
}

fn main() {
    let answer = do_twice(add_one, 5);
    println!("The answer is {}", answer);
}
