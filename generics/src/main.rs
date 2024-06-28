// fn square(x: i32) -> i32 {
//     x*x
// }
//
// fn squaref32(x: f32) -> f32 {
//     x*x
// }
//
// fn main() {
//     println!("The square of the number is {}", square(5));
//     println!("The square of the number is {}", squaref32(5.5));
// }
//

// fn square<T> (x: T) -> T
// where T: std::ops::Mul<Output = T> + Copy> {
//     x*x
// }
//
// fn main() {
//     println!("The square of the number is {}", square(5));
//     println!("The square of the number is {}", square(5.5));
//
// }
//

struct Point<T, U> {
    x: T,
    y: U,
}

impl <T, U> Point<T, U> 
where T: std::fmt::Debug, U: std::fmt::Debug {
    fn printing(&self) {
        println!("The value of the point coordinates are ({:?}, {:?})", self.x, self.y);
    }
}

fn main() {
    let p1: Point<i32, i32> = Point { x: 5, y: 5};
    let p2: Point<f32, f32> = Point { x: 5.5, y: 5.5};
    let p3: Point<i32, f32> = Point { x: 5, y: 5.5};

    p1.printing();
    p2.printing();
    p3.printing();
}
