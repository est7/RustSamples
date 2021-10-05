#![allow(dead_code)]

use std::cmp::PartialOrd;

pub fn main() {}
/*
泛型能够将公共参数抽取出来, 相当于对类型的抽象
实现方在提前实现某个方法时是有矛盾的,他知道调用方会传递某种类型的参数给他,但是因为还没有调用,所以他也不知道类型是什么
所以实现方更加期待的是现在不管这个参数的类型了, 我先把能做的做了, 等到调用方调用时再明确类型是什么

但是因为不确定泛型的类型, 所以这个类型的一些功能就不可能预先知道
这时就需要一些约束, 或者说一个保证

trait能够提供这种保证, 特定的trait能够让给定的泛型保证具备某些特征
而这些特征是生产者在生产的时候需要调用的

泛型是为了编写更容易抽象的代码
而trait是为了限制抽象的边界

同时因为有了trait bound, 所以编译器能够提前检查类型, 规避运行时错误
*/


fn impl_generic() {
    struct Test<T> {
        value: T,
    }

    impl Test<i32> {
        fn print(&self) {
            println!("for i32: {}", &self.value);
        }
    }

    impl Test<f64> {
        fn print(&self) {
            println!("for f64: {}", &self.value);
        }
    }

    Test { value: 1 }.print();
    Test { value: 1.1 }.print();
}


#[allow(unused_variables)]
fn enum_generic() {
    enum Error<T> {
        Fine(T),
        Ops,
    }

    let fine = Error::Fine("str");
    let ops: Error<&str> = Error::Ops;
}

#[allow(unused_variables)]
fn struct_generic() {
    struct Point<T> {
        x: T,
        y: T,
    }

    let a = Point { x: 1, y: 2 };
    let b = Point { x: 1.0, y: 2.0 };
}

fn convert_to_generic() {
    fn largest_v1(list: &[i32]) -> Option<i32> {
        if list.len() <= 0 {
            return None;
        }
        let mut result = i32::MIN;
        for &element in list {
            if element > result {
                result = element
            };
        }
        return Some(result);
    }

    fn largest_v2<T: PartialOrd>(list: &[T]) -> Result<&T, String> {
        if list.len() <= 0 {
            return Result::Err("list is empty".to_string());
        }
        let mut result = &list[0];
        for item in list {
            if *item > *result {
                result = item;
            }
        }
        return Result::Ok(result);
    }

    fn largest_v3<T: PartialOrd + Copy>(list: &[T]) -> Result<T, String> {
        if list.len() <= 0 {
            return Result::Err("list is empty".to_string());
        }
        let mut result = list[0];
        for &item in list {
            if item > result {
                result = item;
            }
        }
        return Result::Ok(result);
    }

    let list: Vec<i32> = vec![1, 2, 3, 4];
    let slice = &list[..];
    match largest_v3(slice) {
        Ok(value) => println!("largest: {}", value),
        Err(msg) => println!("{}", msg),
    }
}

