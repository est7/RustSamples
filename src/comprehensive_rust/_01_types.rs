#![allow(dead_code)]

pub fn main() {}


fn sample_types() {
    //变量默认是不可变的
    let x = 5;
    println!("The value of x is: {x}");
    //❌
    //x = 6;
    println!("The value of x is: {x}");
    // 可以在变量名前添加 mut 来使其可变
    let mut y = 5;
    y = 6;

    let aint: i64 = 213i64;
    let bint = 2_000;

    let astring = "abc";
    //安全引用
    let bstring = r#"\\abc"#;

    //char 就是一个字符
    let mut achar: char = 'a';
    //❌
    //let mut abchar: char = 'ab';
    let mut bchar: &str = "a";
}

/// 复合类型（compound type）可以将多个值组合成一个类型。
/// Rust 有两种基本的复合类型：元组（tuple）和数组（array）。
fn compound_types() {
    let mut b: [i8; 10] = [42; 10];
    b[5] = 0;
    println!("b: {:?}", b);
    println!("b:{:?},{:?}", b, b);

    let x: (i32, f64, u8) = (500, 6.4, 1); // 元组
    let five_hundred = x.0; // 点标记法访问
    let six_point_four = x.1;
    let one = x.2;

    // 数组长度不变，vector 长度可变
    let a = [1, 2, 3, 4, 5]; // 数组
    let first = a[0];
    let second = a[1];

    a[99] // index out of bound
    // RUST_BACKTRACE = 1
}

