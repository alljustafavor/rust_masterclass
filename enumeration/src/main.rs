/* enum Conveyance {
    Car(i32),
    Train(i32),
    Air(i32),
}

impl Conveyance {
    fn travel_allowance(&self) -> f32 {

        let allowance = match self {
            Conveyance::Car(miles) => *miles as f32 * 14.0 * 2.0,
            Conveyance::Train(miles) => *miles as f32 * 18.0 * 2.0,
            Conveyance::Air(miles) => *miles as f32 * 30.0 * 2.0,
        };
        allowance
    }
}

fn main() {
    let participant_1: Conveyance = Conveyance::Car(60);
    let participant_2: Conveyance = Conveyance::Train(120);
    let participant_3: Conveyance = Conveyance::Air(60);
    println!("The participant 1 has a travel allowance of {}", participant_1.travel_allowance());
    println!("The participant 2 has a travel allowance of {}", participant_2.travel_allowance());
    println!("The participant 3 has a travel allowance of {}", participant_3.travel_allowance());
} */
//////////////////////////////////////////////////////////////////////////////////////////////////////
/* 
#[derive(Debug)]
enum Value {
    Integer(i32),
    Float(f32),
}

fn main() {
    let some_val = vec![Value::Integer(12), Value::Float(15.5)];
    println!("The value of the integer is {:?} and the float is {:?}", some_val[0], some_val[1]);

    for i in some_val.iter() {
        match i {
            Value::Integer(num) => println!("The value of the integer is {}", num),
            Value::Float(num) => println!("The value of the float is {}", num),
        }
    }
} */
/////////////////////////////////////////////////////////////////////////////////////////////////////////

// Option ENUM
/* 
fn main() {

    let mut disease: Option<String> = None;
    disease = Some(String::from("Diabetes"));

    match disease {
        Some(disease_name) => println!("You have the disease of {}", disease_name),
        None => println!("You dont have any disease"),
    }
    
} */

/* struct Person {
    name: String,
    age: i32,
}

fn main() {
    let s1 = Some("Some String");
    println!("the value of s1 is {:?} and the value of s1 itself after teh unwrapping is {:?}", s1, s1.unwrap());

    let f1 = Some(10.54);
    let mut f2 = 16.5;
    f2 = f2 + f1.unwrap();
    println!("The values of the sum is {}", f2);

    let v1 = Some(vec![0,1,2,3]);
    let p1 = Person {
        name: String::from("Joe"),
        age: 32,
    };
    let someone = Some(p1);
} */

/* fn square(num: Option<i32>) -> Option<i32> {
    match num {
        Some(number) => Some(number * number),
        None => None,
    }
}
fn main() {
    let number = Some(6);

    if square(number) != None {
        println!("The result of the square is {:?}", square(Some(6)).unwrap());
    } else {
        println!("We do not have any value");
    }
    
} */
///////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////

// Result ENUM

 fn division(divident: f64, divisor: f64) -> Result<f64, String> {
/*     if divisor == 0.0 {
        Err(String::from("Error: Division by zero"))
    } else {
        Ok(divident/ divisor)
    } */


    match divisor {
        0.0 => Err(String::from("Error: division by zero")),
        _ => Ok(divident / divisor),
    }
}

fn main() {
    println!("{:?}", division(9.0, 3.0));
    println!("{:?}", division(9.0, 0.0));

    let some_vec = vec![5,5,2,1,5,9];
    let result_1 = match some_vec.get(5) {
        Some(a) => Ok(a),
        None => Err("The value does not exist"),
    };

    println!("The value of result is {:?}", result_1);
}
