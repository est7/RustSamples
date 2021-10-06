#![allow(dead_code)]

use std::thread;
use std::time::Duration;

pub fn main() {}

fn get_context_value() {
    // trait Fn
    let a = 10;
    // 捕获环境中的不可修改引用
    let b = || a + 1;
    b();
    println!("a: {}", a);

    // trait FnMut
    let mut c = String::from("abc");
    // 捕获环境中的可修改引用
    let mut d = || c.push_str("d");
    d();
    println!("c: {}", c);

    // trait FnOnce
    let e = String::from("abc");
    // 捕获环境中的带所有权的数据
    let f = || e;
    f();
    // println!("e: {}", e);
}


fn create_closure() {
    let println = |a| println!("{}", a);

    let sleep = |sec| {
        println("waiting ");
        let duration = Duration::from_secs(sec);
        thread::sleep(duration);
        println("ok!");
    };

    sleep(2);
}

fn closure_in_struct() {
    struct ClosureCache<V, T: Fn(V) -> V> {
        closure: T,
        value: Option<V>,
    }

    impl<V, T: Fn(V) -> V> ClosureCache<V, T> {
        pub fn new(closure: T) -> Self {
            ClosureCache { closure, value: None }
        }
    }

    impl<V, T> ClosureCache<V, T> where V: Copy, T: Fn(V) -> V {
        pub fn copy(&mut self, input: V) -> V {
            return match self.value {
                Some(v) => v,
                None => {
                    let v = (self.closure)(input);
                    self.value = Some(v);
                    v
                }
            };
        }
    }

    let mut cache = ClosureCache::new(|str| str);
    println!("result: {}", cache.copy("str"));
}

fn closure_argument() {
    fn argument(closure: fn(i32)) {
        closure(10);
    }

    argument(|a| println!("{}", a));
}

fn closure_return() {
    fn call(closure: fn()) {
        println!("start");
        closure();
        println!("end");
    }

    println!("outer start");
    call(|| {
        return;
    });
    println!("outer end");
}
