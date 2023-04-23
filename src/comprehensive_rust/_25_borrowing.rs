#![allow(dead_code)]

pub fn main() {
    // references_borrowing_01();
    // references_borrowing_02();
    // references_borrowing_03();
    // references_mut_borrowing();
    // slice_references();
}

//-----------------------错误示范-------------------------------
fn references_borrowing_01() {
    let s1 = String::from("hello");

    let len = calculate_length_01(s1);

    // println!("The length of '{}' is {}.", s1, len);
    //❌不可以使用s1,因为这时候s1的所有权已经被交给calculate_length_02内部的s,
    //此时要想用的话必须通过转移返回值的所有权,可以看references_borrowing_01
}

fn calculate_length_01(s: String) -> usize {
    s.len()
}

//-----------------------正确示范-------------------------------
//通过转移返回值的所有权

fn references_borrowing_02() {
    let s1 = String::from("hello");

    let (s2, len) = calculate_length_02(s1);

    println!("The length of '{}' is {}.", s2, len);
}

fn calculate_length_02(s: String) -> (String, usize) {
    let length = s.len(); // len() 返回字符串的长度

    (s, length)
}


//-----------------------正确示范-------------------------------
//通过引用的方式,让入参的不获取所有权

fn references_borrowing_03() {
    let s1 = String::from("hello");

    let len = calculate_length_03(&s1);

    println!("The length of '{}' is {}.", s1, len);
}

fn calculate_length_03(s: &String) -> usize {
    s.len()
}


fn references_mut_borrowing() {
    let mut s = String::from("hello");

    let r1 = &mut s;

    println!("{}", r1);

    let r2 = &mut s;
    println!("{}", r2);

    // println!("{}, {}", r1, r2);


    // let r3 = s;


    let x = String::from("world");
    let x1 = &x;
    let x2 = &x;
}


fn slice_references() {
    let mut s = String::from("hello world");

    let word = first_word(&s);

    //s.clear(); //❌ 错误！,s是不可变,word是可变,在不可变中间使用了s延伸来的word

    println!("the first word is: {}", word);
}

fn first_word(s: &String) -> &str {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[0..i];
        }
    }

    &s[..]
}