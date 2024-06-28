struct Person {
    citizenship: String,
    name: String,
    age: i32,
    gender: char,
    salary: i32,
}

impl Person {
    fn new() -> Person {
        Self {
            citizenship: String::from("USA"),
            name: String::from("ABC"),
            age: 40,
            gender: 'M',
            salary: 35_000,
        }
    }

   fn compute_taxes(&self) -> f32 {
        (self.salary as f32 / 3.) * 0.5
    }
}

struct Numbers(i32,i32);

impl Numbers {
   fn greater(&self) -> i32 {
        if self.0 >= self.1 {self.0} else {self.1}
    } 
}

fn main() {
    let person1 = Person{
        name: String::from("Joseph"),
        citizenship: String::from("United States"),
        age: 25,
        gender: 'M',
        salary: 40_000,
    };

    println!("The structure values are {} {} {} {}", person1.citizenship, person1.age, person1.gender, Person::compute_taxes(&person1));
    let person2 = Person::new();
    
    println!("The person 2 is initialized from some defaults. The fields values are");
    println!("Person name {}, Citizenship {}", person2.name, person2.citizenship);

    let person3 = Person {
        age: 50,
        name: String::from("Kamran"),
        ..person1
    };

    println!("the name of person 3 is {}, and is salary is {}", person3.name, person3.salary);

    let mut person4 = Person::new();
    println!("The default name of person 4 is {}", person4.name);
    person4.name = String::from("Asif");
    println!("The undated name of the person is {}", person4.name);

    let some_nums = Numbers(32,16);
    println!("The values of the two fields are {} and {}", some_nums.0, some_nums.1);

    println!("{} is greater", some_nums.greater());
    
}
