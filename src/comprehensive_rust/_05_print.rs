#![allow(dead_code)]

pub fn main() {
    // print1();
}

fn print1() {
    let mut a: [i8; 10] = [42; 10];
    a[5] = 0;
    println!("a: {:?}", a);
    let t: (i8, bool) = (7, true);
    println!("1st index: {0}, 2nd index: {1}", t.0, t.1);
    println!(" {t:?}");
    println!(" {t:#?}");
}

