// struct Demo1;
// struct Demo2;
// struct Demo3;
//
// impl Demo1 {
//     fn m1(){
//         println!("demo1::m1")
//     }
// }
//
// impl Demo2 {
//     fn m2(){
//         println!("demo2::m2")
//     }
// }
//
// impl Demo3 {
//     fn m3(){
//         println!("demo3::m3")
//     }
// }
//
//
// enum EnumTypes {
//     One(Demo1),
//     Two(Demo2),
//     Three(Demo3)
// }
//
// fn enum_method() {
//     let vec: Vec<EnumTypes> = vec![EnumTypes::One(Demo1), EnumTypes::Two(Demo2), EnumTypes::Three(Demo3)];
//
//     for v in vec.iter() {
//         match v {
//             EnumTypes::One(Demo1) =>  Demo1::m1(),
//             EnumTypes::Two(Demo2) =>  Demo2::m2(),
//             EnumTypes::Three(Demo3) =>  Demo3::m3()
//         }
//     }
// }
//
// trait Tmethod {
//     fn m(&self);
// }
//
// impl Tmethod for Demo1 {
//     fn m(&self) {
//         println!("demo1::m1")
//     }
// }
//
// impl Tmethod for Demo2 {
//     fn m(&self) {
//         println!("demo1::m2")
//     }
// }
//
// impl Tmethod for Demo3 {
//     fn m(&self) {
//         println!("demo1::m3")
//     }
// }
//
// fn trait_method(){
//     let vec :Vec<Box<dyn Tmethod>> = vec![
//       Box::new(Demo1),
//       Box::new(Demo2),
//       Box::new(Demo3),
//     ];
//
//     for v in vec.iter(){
//         v.m()
//     }
// }

// 第五题
use std::ops::Add;
#[derive(Debug, PartialEq, Clone, Copy)]
struct Demo5 {
    a: i8,
    b: i16,
}

impl Add for Demo5 {
    type Output = Self;
    fn add(self, other: Self) -> Self {
        Demo5 {
            a: self.a + other.a,
            b: self.b + other.b,
        }
    }
}

fn sum_demo5<T: Add<Output = T>>(num1: T, num2: T) -> T {
    num1 + num2
}

fn main() {
    // 第四题
    // enum_method();
    // trait_method();

    // 第五题
    let d1 = Demo5 { a: 10, b: 5 };

    let d2 = Demo5 { a: 5, b: 2 };
    let r = sum_demo5(d1, d2);
    print!("a - b = {:?}", r);
}
