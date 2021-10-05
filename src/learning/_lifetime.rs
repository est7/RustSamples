#![allow(dead_code)]

pub fn main() {}

#[allow(unused)]
fn lifetime_static() {
    #[derive(Debug)]
    struct Test {}

    let a;
    let b;
    {
        // 字符串字面量只是一个String的切片
        // 实际指向的是一个String
        // 而那个String的生命周期很长, 估计是 'static 的
        // 所以这里就算离开了作用域, a也是可以使用的
        let str = "abc";
        a = str;


        // 而这里的结构体, 是一个栈对象
        // 一旦离开作用域, 就挂球了
        let test = Test {};
        b = &test;
    }

    println!("a: {}", a);
    // 所以以下代码会导致编译错误
    // println!("b: {:?}", b);
}

fn lifetime_struct_impl() {
    struct Test<'a> {
        value: &'a str,
    }

    // 当对象方法不需要明确生命周期时, 可以使用 '_ 忽略生命周期参数
    // impl Test<'_> {
    // 当对象方法需要将传入引用返回, 则必须为传入引用加上生命周期参数
    // 这样才能让编译器计算对象的生命周期与传入引用的生命周期间重叠的部分
    impl<'a> Test<'a> {
        fn test(&self, str: &'a str) -> &str {
            str
        }
    }

    let test = Test { value: "str" };
    let result = test.test("abc");
    println!("{}", result);
}

#[allow(unused_variables)]
fn lifetime_sync_with_generic() {
    struct Holder<'sync, T: 'sync> {
        value: &'sync T,
    }

    // 以下代码传入了一个借用, 借用是Sized的, 所以Holder的导出类型也是Sized, 所以可以分配在栈中, 没有编译问题
    let str = "abc".to_string();
    let holder = Holder { value: &str };

    // 以下代码因为传入了一个slice, 而slice不是Sized的, 所以holder不能分配在栈中, 会导致编译失败
    //  let str = "abc".to_string();
    //  let str_ref = str.as_str();
    //  let holder = Holder { value: str_ref };
}


fn lifetime_sync() {
    struct Holder<'sync> {
        value: &'sync str,
    }
    impl ToString for Holder<'_> {
        fn to_string(&self) -> String {
            self.value.to_string()
        }
    }

    let holder: Holder;
    {
        let str = String::from("abc");
        let slice = str.as_str();
        holder = Holder { value: slice };
        println!("{}", holder.to_string())
    }

    // 结构体将自身的生命周期与内部参数绑定了
    // 所以他的生命周期现在是他的借用参数中生命周期最短的那个
    // println!("{}", holder.to_string())
}


#[allow(unused)]
fn lifetime_is_intersection() {
    fn test<'a>(long: &'a str, sort: &'a str) -> &'a str {
        return sort;
    }

    let long = String::from("long");
    let long_ref = long.as_str();

    let result: &str;
    {
        let sort = String::from("sort");
        let sort_ref = sort.as_str();
        result = test(long_ref, sort_ref);
        println!("{}", result);
    };

    // 调用test函数后, 返回的类型是两个入参借用中较短那个
    // 也就是sort_ref指向的对象sort的生命周期
    // 而sort因为离开了作用域, 生命周期已结束
    // 所以result也就不能被调用了
    // println!("{}", result);
}

fn get_slice_longest() {
    // 以下方法签名中,问题出在返回值上, 返回值是一个借用的引用, 但是不确定是借用了哪一个的
    // 这就导致了这个方法返回的引用生命周期不明确
    // 编译器就无法执行生命周期检查, 有潜在的风险
    // fn longest(first: &str, second: &str) -> &str {

    // 使用以下方法签名, 添加了生命周期限制, 强制要求两个入参的生命周期一样长, 并且返回的参数也是
    // 但是实际上两个入参的生命周期是不一样长的
    // 所以可以理解 'a 其实是两个入参生命周期中较短的那个, 编译器会自动推导出来
    // 可以理解为两个生命周期的交集,
    fn longest<'a>(first: &'a str, second: &'a str) -> &'a str {
        return if first.len() > second.len() {
            first
        } else {
            second
        };
    }

    let sort;
    let s1 = String::from("xyz");
    sort = &s1;

    let long;
    let s2 = String::from("abcd");
    long = &s2;

    let result: &str = longest(long, sort);
    println!("{}", result);
}

#[allow(unused)]
fn error() {
    let r: &i32;
    {
        let x = 5;
        r = &x;     // 产生了借用
    }               // 此处x离开作用域, 被销毁

    // 下方的调用中, r的声明周期长于x, 长生命周期持有短生命周期是不允许的, 编译失败
    // println!("{}", r);
}