use std::collections::HashMap;
/*
fn main() {

    let mut person:HashMap<&str, i32> = HashMap::new();
    person.insert("Joseph", 25);
    person.insert("Austin", 24);
    person.insert("Jody", 25);

    println!("The age is {:?}", person.get("Joseph").unwrap());

    if person.contains_key("Joseph") {
        println!("The value exist");
    } else {
        println!("The value does not exist");
    }

    match person.get("Joseph") {
        Some(value) => println!("The value exist {}", value),
        None => println!("The value does not exist"),
    }

    for (name , age) in &person {
        println!("The person {} is {} years old.", name, age);
    }
} */

///////////////////////////////////////////////////////////////////

/* fn main() {
    let mut likes:HashMap<&str , &str> = HashMap::new();
    // likes.insert("Joseph", "apple");
    // likes.insert("Joseph", "mango");
    // println!("The fruit which liked is {:?}", likes)

    likes.entry("Joseph").or_insert("apple");
    likes.entry("Joseph").or_insert("mango");

    println!("The fruit which is liked is {:?}", likes);
} */

///////////////////////////////////////////////////////////////////

fn main() {
    let some_vec = vec![5,5,8,8,1,0,1,5,5,5,5];
    let mut freq_vec:HashMap<i32,u32> = HashMap::new();

    for i in &some_vec {
        let freq: &mut u32 = freq_vec.entry(*i).or_insert(0);
        *freq += 1;
    }
    println!("{:?}", freq_vec);
}
