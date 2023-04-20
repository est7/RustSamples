#![allow(dead_code)]

pub fn main() {}


fn method() {
    let mut v = Vec::new();
    v.push((10, false));
    v.push((20, true));
    println!("v: {v:?}");

    let vv = v.iter().collect::<std::collections::HashSet<_>>();

    println!("vv: {vv:?}");
}


fn variables() {
    println!("Hello, world!");

    let x = 5; // let mut x = 5;
    println!("The value of x is: {}", x);
    // ❌ 是不可变变量 这里会报错，cannot assign twice to immutable variable
    //x = 6;
    println!("The value of x is: {}", x);


    // 常量， 命名规范 大写字母加下划线
    const THREE_HOURS_IN_SECONDS: u32 = 60 * 60 * 3;

    // 遮蔽/隐藏（Shadowing）
    let x = x + 1;
    {
        let x = x * 2;
        println!("The value of x in the inner scope is: {}", x);
    }
    println!("The value of x is: {}", x);

    let spaces = "   "; // 字符串切片
    let spaces: usize = spaces.len(); // usize 类型 arch
    println!("The value of spaces is: {}", spaces);
}