use std::{fs, io};
use std::fmt::Error;
use std::fs::File;
use std::io::{ErrorKind, Read};

pub fn main() {}

fn read_file_return_result() {
    fn read_file_content_v1(path: &str) -> Result<String, io::Error> {
        let file = File::open(path);
        let mut file = match file {
            Ok(file) => file,
            Err(e) => return Err(e),
        };
        let mut content = String::new();
        let result = file.read_to_string(&mut content);
        return match result {
            Ok(_) => Ok(content),
            Err(e) => Err(e),
        };
    }

    fn read_file_content_v2(path: &str) -> Result<String, io::Error> {
        let mut content = String::new();
        File::open(path)
            ?.read_to_string(&mut content)
            ?;
        Ok(content)
    }

    fn read_file_content_v3(path: &str) -> Result<String, io::Error> {
        fs::read_to_string(path)
    }

    let path = "/Volumes/Crucial/codes/Rust/collections/src/learning/mod.rs";
    let content = read_file_content_v3(path).unwrap_or_else(|err| {
        return err.to_string();
    });

    println!("{}", content)
}

#[allow(unused)]
fn panic_unwrap() {
    // unwrap直接panic并退出
    let file = File::open("cba").unwrap();
    // expect可以在panic信息中添加说明
    let file = File::open("abc").expect("open file error!");
}

#[allow(unused_variables)]
fn open_file_with_lambda() {
    let file = File::open("test.txt").unwrap_or_else(|err| {
        if err.kind() == ErrorKind::NotFound {
            panic!("not found")
        } else {
            panic!("other error")
        }
    });
}

fn open_file_with_match() {
    match File::open("test.txt") {
        Ok(file) => file,
        Err(e) => match e.kind() {
            ErrorKind::NotFound => panic!("file not found"),
            _ => panic!("unknown error"),
        }
    };
}