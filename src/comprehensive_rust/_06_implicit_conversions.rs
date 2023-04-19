#![allow(dead_code)]

pub fn main() {
    // method()
}

fn multiply(x: i16, y: i16) -> i16 {
    x * y
}

fn method() {
    let x: i8 = 15;
    let y: i16 = 1000;


    //❌ 不能进行隐士转换
    // println!("{x} * {y} = {}", multiply(x, y));
    println!("{x} * {y} = {}", multiply(i16::from(x), y));
    println!("{x} * {y} = {}", multiply(x.into(), y));
}

