#![allow(dead_code)]


pub fn main() {
    let mut vec: Vec<i32> = Vec::new();
    let mut v = vec![1, 2, 3];
    vec.push(1);
    v.push(8);


    let v = vec![1, 2, 3, 4, 5];

    let does_not_exist = &v[100];
    let does_not_exist = v.get(100);


}

