//fn main() {
    /* let i: &i32;
    {
        let j = 5;
        i = &j
    }
    println!("The value of i = {}", i); */

    /* let some_int = 10;
    let additional_int = some_fn(&some_int);
    println!("{}", additional_int); */
    /* 
    let int1 = 5;
    let int2 = 10; */

/*     let s_1: &str = "Hello";
    let v: &str;
    {
        let s_2: String = String::from("World");
        v = some_fn(s_1, s_2.as_str());
    }
    println!("{}", v);

}

fn some_fn<'a,'b>(first_str: &'a str, second_str: &'b str) -> &'a str {
    first_str
} */

/* fn greater(i: &i32, j: &i32) -> i32 {
    if i > j {
        i
    } else {
        j
    }
} */

/* fn some_fn(i: &i32) -> &i32 {
    &i
} */
/* 
fn main() {
    let int1 = 5;
    let int2 = 10;
    let result = greater(&int1, &int2);
}


fn greater<'a>(i: &'a i32, j: &i32) -> &'a i32 {
    i 
} */

/* fn main() {
    let int1 = 5;
    {
        let int2 = 10;
        let result = greater(&int1, &int2);
        println!("The larger value is {}", result);
    }
}

fn greater<'a,'b>(i: &'a i32, j: &'a i32) -> &'a i32 {
    if i > j {
        i
    } else { j
    }
} */

/* struct Person<'a> {
    name: &'a str,
    age:  i32,
}

fn main() {
    let first_name = "Joseph";
    let mut joseph = Person {
        name: &first_name,
        age: 25,
    };
    {
        let last_name = String::from("Noblett");
        joseph.name = &last_name;
    }
    println!("The name of the person is {} and is age is", josesph.name, joseph.age);

} */

/* fn main() {
    let some_vec = vec![5,8,9,8,7,5,2];
    let return_vec = use_vec(&some_vec, &some_vec);
    println!("The values are {:?}", return_vec);
}

fn use_vec<'a>(vec1: &'a [i32], vec2: &'a [i32]) -> &'a [i32] {
    if 3 > 5 {
        vec1
    } else {
        vec2
    }
} */

fn main() {
    let str_1 = "some str";
    let recieved_str = return_str(&str1);
}

fn return_str<a'>(s_1: &'a str) -> &'a str {
    s_1
}
