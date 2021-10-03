use std::cmp::PartialOrd;
use std::collections::hash_map::Values;

pub fn main() {
    impl_generic();
}

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
    fn largest<T: PartialOrd>(list: &[T]) -> Result<&T, String> {
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

    fn largest_number(list: &[i32]) -> Option<i32> {
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

    let list: Vec<i32> = vec![1, 2, 3, 4];
    let slice = &list[..];
    match largest(slice) {
        Ok(value) => println!("largest: {}", value),
        Err(msg) => println!("{}", msg),
    }
}

