#![allow(dead_code)]

use std::ops::Deref;

pub fn main() {}

fn deref_string() {
    // box比较牛逼的一点就是对他的解引用能够产生移动
    let string = Box::new("abc".to_string());
    let mut a = *string;
    a.push_str("abc");

    // 换个角度思考, 为什么自定义的Ref不行?
    // 猜测是因为实际上box返回的引用是指向了堆中的string的指针
    // box持有的不是string, 所以移动的对象不是string, 而是指向堆的那个指针的所有权
    // string一直待在堆中, 所有权一直没发生改变
    // 而指针, 是实现了Copy的!!!
}


fn dereference_impl_string() {
    struct Ref<T> (T);
    impl<T> Ref<T> {
        pub fn new(inbox: T) -> Ref<T> { Ref(inbox) }
    }

    impl<T> Deref for Ref<T> {
        type Target = T;

        fn deref(&self) -> &T {
            let self_ref = self;
            let reference = &self_ref.0;
            return reference;
        }
    }

    // 生产了一个带所有权的对象, 所有权移动给了string
    let string = "abc".to_string();
    // 创建一个Ref对象
    // 把string的所有权移动给Ref
    let reference = Ref::new(string);
    // 这里隐式地调用了一次deref方法
    // 相当于 *(reference.deref())
    // 所以下方的代码是有问题的, 他想通过一个应用获取所有权
    // let string = *reference;
    let deref = reference.deref().as_str();
    println!("abc:{}", deref);

    // 为了解决这个问题, 可以传递string的引用, 引用实现了Copy,
    let string_ref = &"abc".to_string();
    let reference = Ref::new(string_ref);
    let deref = *reference;
    // 但是由于这个引用是不可修改的, 所以无法修改这个值
    // deref.push_str("def");
    println!("{}", deref);
}


fn dereference_impl_i32() {
    struct MyBox<T> (T);
    impl<T> MyBox<T> {
        pub fn new(inbox: T) -> MyBox<T> { MyBox(inbox) }
    }

    impl<T> Deref for MyBox<T> {
        // 这里定义了他返回引用的类型
        type Target = T;

        // 这里返回一个引用
        // 而且deref是一个方法, 它隐式地获取了自己的引用
        fn deref(&self) -> &T {
            &self.0
        }
    }

    let abc = 5;
    let pointer = MyBox::new(abc);

    // 这里实际上时候copy了一个对象出来了
    let deref = *pointer;
    println!("{}", deref);

    // 因为解引用(移出)后, 仍然能够正常使用abc
    println!("old {}", abc);
}

#[allow(unused)]
fn reference_i32() {
    let a = 5;
    let x = &a;
    // 猜测
    // 对已i32 解引用相当于copy一个出来, 所以所有权不会移动
    let mut b = *x;

    // 修改解引用后的值,对于i32来说不会影响之前的a
    b = 1;
    assert_eq!(5, a);
    assert_eq!(1, b);
    // 所以实际上对i32解引用不会产生移动, 而是直接Copy了

    // 所以可以反复解引用 &i32 而不会出错
    let c = *x;
    println!("c: {}", c);

    // 但是对于String来说, 就不是这样的了, 它无法通过编译
    let string = "abc".to_string();
    let str = &string;
    // let deref_string = *str;

    // 提示是没有实现Copy
    // 也就是说i32实际上实现了Copy, 在解引用的时候是Copy了一个新的带所有权的对象
}

fn reference_string() {
    let string = "abc".to_string();
    let pointer = Box::new(string);

    // 猜测
    // 这里的解引用把所有权让度出来, 本质上是对 dereference函数 实现起的作用
    // 调用 *pointer, 相当于pointer.dereference()
    let mut deref_string = *pointer;

    // 修改
    deref_string.push_str("d");
    println!("deref_string: {}", deref_string);

    // 重新解引用
    // let new_ref = *pointer;
    // println!("new_ref: {}", new_ref);
}


fn cons_list_ref() {
    // 实际上不用Box也是可以做到的
    // 本质上是因为Box是对解引用 deref() 的定义
    // 而引用天生支持解引用
    enum List<'a, E> {
        Next(E, &'a List<'a, E>),
        Nil,
    }

    let list = List::Next(
        18,
        &List::Next(
            19,
            &List::Nil));

    fn print(list: &List<'_, i32>) {
        if let List::Next(value, next) = list {
            println!("{}", value);
            print(next);
        }
    }
    print(&list);
}


#[allow(unused)]
fn cons_list() {
    enum List<E> {
        Next(E, Box<List<E>>),
        Nil,
    }

    let list = List::Next(
        18,
        Box::new(List::Next(
            19,
            Box::new(List::Nil))),
    );
}

