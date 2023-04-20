#![allow(dead_code)]

use std::ops::Add;

pub fn main() {
    //ownership();

    ownership_string();
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
