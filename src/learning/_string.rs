#![allow(dead_code)]

pub fn main() {}

fn add_append_format() {
    let s1 = "abc".to_string();
    let s2 = "_".to_string();
    let s3 = "xyz".to_string();
    let s4 = s1 + &s2 + &s3;

    println!("{}", s4);

    let str = String::new() + "a" + "b" + "c";
    println!("{}", str);

    let format = format!("{},{},{}", "a", "bb", "ccc");
    println!("{}", format);
}

fn string_slice() {
    fn get_word(string: &String) -> usize {
        let bytes = string.as_bytes();
        for (i, &byte) in bytes.iter().enumerate() {
            if byte == b' ' {
                return i;
            }
        }
        return bytes.len();
    }

    fn slice(string: &String) -> &str {
        let bytes = string.as_bytes();
        for (i, &byte) in bytes.iter().enumerate() {
            if byte == b' ' {
                return &string[..i];
            }
        }
        return &string[..];
    }

    #[allow(unused)]
        let mut string = String::from("abc ddd");
    #[allow(unused)]
        let slice = slice(&string);
    // 以下代码尝试获取修改引用, slice本身是string的不可修改引用, 不能同时获取, 编译出错
    // string.clear();
    println!("slice: {}", slice)
}

fn string_append() {
    let mut string = String::new();
    string.push_str("a");
    string.push_str("b");

    println!("{}", string);
}


