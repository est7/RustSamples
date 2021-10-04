use std::fmt::Display;

pub fn main() {}

fn trait_for_generic() {
    pub trait CanPrint {
        fn print(&self);
    }

    // 这个是直接作用于泛型的特征实现,
    // 只要这个泛型的对象有ToString实现, 它就能调用print
    // 相当于特征的继承: CanPrint继承自ToString, 同时额外提供print方法
    impl<T: ToString> CanPrint for T {
        fn print(&self) {
            println!("{}", self.to_string())
        }
    }

    // 这时&str因为实现自ToString. 所以也实现了CanPrint
    "abc".print()
}


#[allow(unused_variables)]
fn trait_condition() {
    struct Provider<T> {
        value: T,
    }

    pub trait Copiable<T> {
        fn copy(&self) -> T;
    }

    impl<T> Provider<T> {
        pub fn new(value: T) -> Self {
            Provider { value }
        }

        pub fn value(&self) -> &T {
            &self.value
        }

        pub fn set_value(&mut self, value: T) {
            self.value = value;
        }
    }

    // 这里限定了被实现的对象的泛型,
    // 如果实现的泛型没有实现Copy, 这不能调用copy方法,
    // 例如String
    impl<T: Copy> Copiable<T> for Provider<T> {
        fn copy(&self) -> T {
            let copy = self.value;
            return copy;
        }
    }

    let provider = Provider::new(10);
    let copied = provider.copy();
    println!("copied: {}, provider:{}", copied, provider.value());


    let provider = Provider::new("abc".to_string());
    // 因为String没有Copy的特征, 所以调用copy方法会导致编译失败
    // let copied = provider.copy();
}

fn trait_return() {
    pub trait Runnable {}

    impl Runnable for i32 {}
    impl Runnable for i64 {}

    // 下面的代码无法通过编译
    // fn return_runnable() -> impl Runnable {
    //     return if true {
    //         1
    //     } else {
    //         1.1
    //     };
    // }
}

#[allow(unused_variables)]
fn trait_where() {
    pub trait Bound {}
    pub trait Around {}

    fn test<T, A>(t: T, a: A)
        where T: Bound,
              A: Around {}
}

#[allow(unused_variables)]
fn trait_add() {
    pub trait Bound {}
    pub trait Around {}

    fn call_bound_or_around(a: impl Bound + Around) {}
    fn bound_way<T: Bound + Around>(t: T) {}
}

fn trait_bound() {
    pub trait Bound {
        fn go(&self);
    }

    impl Bound for i32 {
        fn go(&self) {
            println!("go: {}", self)
        }
    }

    fn use_bound<B: Bound>(b: B) {
        b.go()
    }

    use_bound(1024);
}

fn test() {
    pub trait Runnable {
        fn run(&self);
    }

    impl Runnable for i32 {
        fn run(&self) {
            println!("run {}", self)
        }
    }

    fn call_run(runnable: impl Runnable) {
        runnable.run()
    }

    call_run(12);
}