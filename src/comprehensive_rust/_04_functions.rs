#![allow(dead_code)]

use std::f32::RADIX;

pub fn main() {
    // fizzbuzz_to(20);
    //method();
    // expression();
}

fn fizzbuzz_to(n: u32) {  // `-> ()` is normally omitted
    for i in 1..=n {
        fizzbuzz(i);
    }
}

fn is_divisible_by(lhs: u32, rhs: u32) -> bool {
    if rhs == 0 {
        return false;  // Corner case, early return
    }
    lhs % rhs == 0     // The last expression in a block is the return value
}

fn fizzbuzz(n: u32) -> () {  // No return value means returning the unit type `()`
    match (is_divisible_by(n, 3), is_divisible_by(n, 5)) {
        (true, true) => println!("fizzbuzz"),
        (true, false) => println!("fizz"),
        (false, true) => println!("buzz"),
        (false, false) => println!("{n}"),
    }
}


struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    fn area(&self) -> u32 {
        self.width * self.height
    }

    fn inc_width(&mut self, delta: u32) {
        self.width += delta;
    }
}

/// old area: 50
/// new area: 75
fn method() {
    let mut rect = Rectangle { width: 10, height: 5 };
    println!("old area: {}", rect.area());
    rect.inc_width(5);
    println!("new area: {}", rect.area());
}

fn overloading() {
    println!("coin toss: {}", pick_one("heads", "tails"));
    println!("cash prize: {}", pick_one(500, 1000));
}

fn pick_one<T>(a: T, b: T) -> T {
    if std::process::id() % 2 == 0 { a } else { b }
}


fn expression() {
    let y = {
        let x = 3;
        x + 1
    };
    println!("The value of y is: {y}");
}

fn five() -> i32 {
    return 45;
    // 或者直接 45
}