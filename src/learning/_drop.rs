#![allow(dead_code)]

use std::mem;

pub fn main() {}

#[allow(unused_variables)]
fn drop_early() {
    struct Test(String);

    impl Drop for Test {
        fn drop(&mut self) {
            println!("drop: {}", self.0)
        }
    }

    {
        let a = Test("a".to_string());

        let b = Test("b".to_string());
        mem::drop(b);
    }

    println!("finish!");
}

fn test_drop() {
    struct Test();

    impl Drop for Test {
        fn drop(&mut self) {
            // 离开作用域时自动调用
            println!("im drop");
        }
    }

    {
        Test();
    }
}