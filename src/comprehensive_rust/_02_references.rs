#![allow(dead_code)]

pub fn main() {
    // dangling_references()
    // ownship_references()
    // borrowing();
    // correct();
}

fn references() {
    let mut x: i32 = 10;
    let ref_x: &mut i32 = &mut x;
    *ref_x = 20;
    println!("x: {x}");
}


//❌,x'被扔掉了，但仍被借用
/*fn dangling_references() {
    let ref_x: &i32;
    {
        let x: i32 = 10;
        ref_x = &x;
    } //x'被扔掉了，但仍被借用
    println!("ref_x: {ref_x}");
}
*/

fn correct() {
    let x: i32 = 10;
    let ref_x: &i32 = &x;

    println!("ref_x: {}", ref_x);
}

fn ownship_references() {
    let s1 = String::from("hello");
    let s2 = s1; // s1 的所有权移动到 s2
    // println!("s1: {}", s1); // 这里会报错，因为 s1 的所有权已经移动到 s2
    println!("s2: {}", s2); // 正确输出 "s2: hello"
}

fn borrowing() {
    let s = String::from("hello");

    let len = calculate_length(&s); // 不可变借用
    println!("The length of '{}' is {}.", s, len);

    let mut s = String::from("hello");
    change(&mut s); // 可变借用
    println!("Changed string: {}", s);
}

fn calculate_length(s: &String) -> usize {
    // s.push_str(", world!");
    s.len()
}

fn change(s: &mut String) {
    s.push_str(", world!");
}

