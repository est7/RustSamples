#![allow(dead_code)]

use std::ops::Add;

pub fn main() {
    //ownership();
    // ownership_string();
    // move_ownership();
    // return_value_for_ownership();
}

fn return_value_for_ownership() {
    let s1 = gives_ownership();         // gives_ownership 将返回值
    // 转移给 s1

    let s2 = String::from("hello");     // s2 进入作用域

    let s3 = takes_and_gives_back(s2);  // s2 被移动到
    // takes_and_gives_back 中，
    // 它也将返回值移给 s3
} // 这里，s3 移出作用域并被丢弃。s2 也移出作用域，但已被移走，
// 所以什么也不会发生。s1 离开作用域并被丢弃

fn gives_ownership() -> String {             // gives_ownership 会将
    // 返回值移动给
    // 调用它的函数

    let some_string = String::from("yours"); // some_string 进入作用域。

    some_string                              // 返回 some_string
    // 并移出给调用的函数
    //
}

// takes_and_gives_back 将传入字符串并返回该值
fn takes_and_gives_back(a_string: String) -> String { // a_string 进入作用域
    //

    a_string  // 返回 a_string 并移出给调用的函数
}

fn ownership_string() {
    let s1 = String::from("hello");
    let s2 = s1;

    //println!("{}, world!", s1);//❌
    println!("{}, world!", s2);
}

fn ownership() {
    let mut s = String::from("hello");
    s.push_str(", world!"); // push_str() 在字符串后追加字面值
    println!("{}", s); // 将打印 `hello, world!`


    let x = 5;
    let y = x; // 编译的时候就确定了大小，在栈上分配的，会自动复制，所以不存在所有权转移的问题。
    println!("x = {}, y = {}", x, y);


    // 数据类型
    let s1 = String::from("hello");
    let s2 = s1; // ！！！！ 【s1 value moved here】
    // println!("The value of s1 is: {}", s1); // ❌会报错 value borrowed here after move
    println!("The value of s2 is: {}", s2);

    let s1 = String::from("hello");
    let s2 = s1.clone(); // 深拷贝

    println!("s1 = {}, s2 = {}", s1, s2);

    let s1 = "abc"; // 字符串字面量
    // let s2 = s1 + "cdf"; // ❌报错 cannot add `&str` to `&str`
    let s2 = s1.to_owned() + "cdf"; // String + &str
    println!("s1 = {} , s2 = {}", s1, s2); // 报错 cannot be used to concatenate two `&str` strings
}


fn move_ownership() {
    let s = String::from("hello");  // s 进入作用域

    takes_ownership(s);             // s 的值移动到函数里 ...
    // ... 所以到这里不再有效
    // println!("{}", s);//❌当尝试在调用 takes_ownership 后使用 s 时，Rust 会抛出一个编译时错误

    let x = 5;                      // x 进入作用域

    makes_copy(x);                  // x 应该移动函数里，
    // 但 i32 是 Copy 的，
    // 所以在后面可继续使用 x
} // 这里，x 先移出了作用域，然后是 s。但因为 s 的值已被移走，
// 没有特殊之处

fn takes_ownership(some_string: String) { // some_string 进入作用域
    println!("{}", some_string);
} // 这里，some_string 移出作用域并调用 `drop` 方法。
// 占用的内存被释放

fn makes_copy(some_integer: i32) { // some_integer 进入作用域
    println!("{}", some_integer);
} // 这里，some_integer 移出作用域。没有特殊之处