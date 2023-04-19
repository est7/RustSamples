#![allow(dead_code)]

pub fn main() {
    // slices()
    // stringVstr();
}


fn slices() {
    // let a: [i32; 6] = [10, 20, 30, 40, 50, 60];
    let mut a: [i32; 6] = [10, 20, 30, 40, 50, 60];
    //println!("a: {a}");// ^ `[i32; 6]` cannot be formatted with the default formatter
    //println!("a: {a:#}");//^ `[i32; 6]` cannot be formatted with the default formatter
    println!("a: {a:?}");
    println!("a: {a:#?}");
    a[2] = 33;

    //
    let s: &[i32] = &a[2..4];
    println!("s: {s:?}");
    //- 如果slice从索引0开始，则Rust的范围语法允许我们省略起始索引，这意味着`&a[0..a.len()]`和`&a[..a.len()]`是相同的。
    // - 对于最后一个索引也是如此，因此`&a[2..a.len()]`和`&a[2..]`是相同的。
    let s1 = &a[..4];
    println!("s: {s1:?}");
    let s2 = &a[2..];
    println!("s: {s2:?}");
}


fn stringVstr() {
    let s1: &str = "World";
    println!("s1: {s1}");

    let mut s2: String = String::from("Hello ");
    println!("s2: {s2}");
    s2.push_str(s1);
    println!("s2: {s2}");

    let s3: &str = &s2[6..];
    println!("s3: {s3}");
}