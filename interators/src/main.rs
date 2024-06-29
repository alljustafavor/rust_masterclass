fn main() {
    /* let some_vec = vec![1,2,3,4,5,6,7];
    let mut iter = some_vec.iter();

    println!("The iterator: {:?}", iter);

    println!("{:?}", iter.next());
    println!("{:?}", iter.next()); */
    let a = vec![0,1,2,3,4,5,6,7];

    let check = a.iter().any(|&x| x > 0);
    println!("The value of the any function is {}", check);

    let check = a.iter().all(|&x| x > 0);
    println!("The value of the all function is {}", check);

    let check = a.iter().find(|&&x| x > 0);
    println!("The value of the find function is {}", check.unwrap());

    let check = a.iter().position(|&x| x > 4);
    println!("The value of the position function is {}", check.unwrap());

    let check = a.iter().rposition(|&x| x > 4);
    println!("The value of the function rposition is {}", check.unwrap());

    let check = a.iter().max();
    println!("The value of the function max is {}", check.unwrap());

    let check = a.iter().min();
    println!("The value of the function min is {}", check.unwrap());

    let iter = a.iter().rev();
    println!("The value of the function rev is {:?}", iter);

    let filtered_values = a.iter().filter(|&x| *x >= 5).collect::<Vec<&u32>>();
    println!("{:?}", filtered_values);

    let b = a.clone();
    let filtered_values = a.into_iter().filter(|&x| x >= 5).collect::<Vec<u32>>();
    println!("{:?}", filtered_values);

    //println!("{:?}", a);

    let mut mapped_values = b.iter().map(|x| 2 * *x).collect::<Vec<u32>>();
    println!("{:?}", mapped_values);

    let mut mapped_values = b.iter().map(|x| 2 * *x).filter(|x| *x > 10).collect::<Vec<u32>>();
    println!("{:?}", mapped_values);

}
