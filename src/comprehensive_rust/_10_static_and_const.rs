#![allow(dead_code)]

pub fn main() {
    // scopes();
    // shadowing();
}

fn scopes() {
    let a = 10;
    println!("before: {a}");

    {
        let a = "hello";
        println!("inner scope: {a}");

        let a = true;
        println!("shadowed in inner scope: {a}");
    }

    println!("after: {a}");
}


fn shadowing() {
    let a = 1;
    let b = &a;
    let a = a + 1;
    println!("{a} {b}");
}