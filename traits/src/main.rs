// struct Person {
//     citizenship: String,
//     name: String,
//     age: u8,
//     gender: char,
//     salary: i32,
// }
//
// struct Student {
//     name_std: String,
//     age: u8,
//     sex: char,
//     country: String,
// }
//
// trait GeneralInfo {
//     fn info(&self) -> (&str,u8,char);
//     fn country_info(&self) -> &str;
// }
//
// impl GeneralInfo for Person {
//     fn info(&self) -> (&str, u8, char) {
//         (&self.name, self.age, self.gender)
//     }
//
//     fn country_info(&self) -> &str {
//         &self.citizenship
//     }
// }
//
// impl GeneralInfo for Student {
//     fn info(&self) -> (&str,u8,char) {
//         (&self.name_std, self.age, self.sex)
//     }
//     
//     fn country_info(&self) -> &str {
//         &self.country
//     }
//     
// }
//
// fn main() {
//     let person1 = Person {
//         name: String::from("Joseph Noblett"),
//         citizenship: String::from("USA"),
//         age: 25,
//         gender: 'M',
//         salary: 40_000,
//     };
//     let student1 = Student {
//         name_std: String::from("Austin Bunn"),
//         age: 15,
//         sex: 'M',
//         country: String::from("USA"),
//     };
//
//     println!("The basic info includes {:?}", person1.info());
//     println!("The basic info for the student is {:?}", student1.info());
//
//     
// }
///////////////////////////////////////////////////////////////////////////////////////
// struct Circle {
//     radius: f32,
// }
//
// struct Rectangle {
//     length: f32,
//     width: f32,
// }
//
// trait GeneralInfo {
//     fn area(&self);
//
//     fn perimeter(&self);
// }
//
// impl GeneralInfo for Circle {
//     fn area(&self) {
//         let area_of_circle = 3.14 * (self.radius * self.radius);
//         println!("The area of the circle is {}", area_of_circle);
//     }
//
//     fn perimeter(&self) {
//         let circumference = 2.0 * 3.14 * self.radius;
//         println!("The circumference of the circle is {}", circumference);
//     }
//     
// }
//
// impl GeneralInfo for Rectangle {
//     fn area(&self) {
//         let area_of_rect = self.length * self.width ;
//         println!("The area of the rectangle is {}", area_of_rect);
//     }  
//
//     fn perimeter(&self) {
//         let perimeter = (self.length * 2.0) + (self.width * 2.0);
//         println!("The perimeter of the rectangle is {}", perimeter);
//     }
// }
//
// fn main() {
//     let rect = Rectangle{
//         length: 100.0,
//         width: 50.0,
//     };
//
//     let cir = Circle{
//         radius: 2.0,
//     };
//
//     rect.area();
//     rect.perimeter();
//     cir.area();
//     cir.perimeter();
//     
// }
/////////////////////////////////////////////////////////////////////////////////////////

struct Data {
    some_data: Vec<i32>
}

trait BasicStats {
    fn mean(&self) -> f32;
    fn variance(&self) -> f32;
}

impl BasicStats for Data {
    fn mean(&self) -> f32 {
        let mut sum: i32 = 0;
        for i in self.some_data.iter() {
            sum += *i
        }
        sum as f32 / self.some_data.len() as f32
   }

    fn variance(&self) -> f32 {
        let mu: f32 = self.mean();
        let mut sum_squared_diff: f32 = 0.0;
        for i in self.some_data.iter() {
            sum_squared_diff += (*i as f32 - mu) * (*i as f32 - mu);
        }
        sum_squared_diff / self.some_data.len() as f32

    }
}

fn main() {
    let my_data = Data {
        some_data: vec![5,6,9,8,7,4,8],
    };
    println!("The mean of the data is {}", my_data.mean());
    println!("The variance of the data is {}", my_data.variance());
}
