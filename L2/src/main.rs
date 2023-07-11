
// 可变引用
fn foo(s: &mut String){
    s.push_str("hhhhhh")
}

// 不可变引用
fn foo1(s: &String){
    println!("{s}")
}

fn main() {
    //不可变引用所有权
    // let a = 10u32;
    // let a = String::from("hhhhh");
    // let b = &a;
    // let c = &&&&a;
    // let d = &b;
    // let e = b;
    // println!("{a}");
    // println!("{b}");
    // println!("{c}");
    // println!("{d}");
    // println!("{e}");

    // 可变引用所有权
    // let mut a = 10u32;
    // // println!("{c}");
    // let b = &mut a;
    // *b = 20;
    // let c = &mut a; // 可变引用所有权overlap
    // println!("{b}");
    // // println!("{b}");

    // 不可辨引用
    // let mut a = String::from("dddd");
    // foo1(&a);
    // println!("{a}")  // a的所有权被释放，无法打印

    // 可变引用
    let mut a = String::from("dddd");
    // foo1(&mut a); // 不可变方法传入可变引用，仍然可以正常运行
    foo(&mut a); //
    println!("{a}")  // 正常运行
}
