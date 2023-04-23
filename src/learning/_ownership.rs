#![allow(dead_code)]

pub fn main() {}

fn dangle_reference() {
    // 返回引用, 但是引用所指的对象已经被回收, 编译不通过
    // fn error() -> &String {
    //     let string = String::from("abc");
    //     &string
    // }
}

#[allow(unused)]
fn data_race() {
    fn two_or_more_pointers_access_the_same_data_at_the_same_time() {
        // 两个或更多指针同时访问同一数据
        let mut string = String::from("mut");
        let mut r1 = &mut string;
        let mut r2 = &mut string;
        // 以下为 两个指针 同时 访问一个数据, 编译不通过
        // println!("r1 {} r2 {}", r1, r2);
    }

    fn test() {
        let mut str = String::from("abc");

        let str_1 = &str;
        let str_2 = &mut str;

        // 总结的规律就是, 声明和调用之间不能隔着一个可修改借用
        // 结构体方法也有可能发生可修改借用,也会导致编译问题
        // println!("{}", str_1)
    }

    fn at_least_one_pointer_is_used_to_write_data() {
        // 至少有一个指针被用来写入数据
        let mut string = String::from("mut");
        let r1 = &string;
        let mut r2 = &mut string;

        r2.push_str("abc");

        // r1
        // println!("{}", *r1)
    }

    // 没有同步数据访问机制
}

fn ownership_borrowing() {
    fn string_length(reference: &String) -> usize {
        println!("{}", reference);
        reference.len()

        // 此处释放reference
    }

    let s: String = String::from("hello");

    // 下方创建了一个引用值
    let reference = &s;

    // ? &?类型 是 &String类型的子类型(逆变)

    // 下方移动了引用值的所有权
    let length = string_length(reference);
    println!("{}, length: {}", s, length)

    // reference所有权已移动,此处不处理
    // 此处释放s
}

fn ownership_return() {
    fn takes_and_gives_ownership(string: String) -> String {
        return string;
    }

    fn gives_ownership() -> String {
        let string = String::from("xyz");
        println!("{}", string);
        return string;
    }

    let s = gives_ownership();
    let s = takes_and_gives_ownership(s);

    println!("{}", s)
}

fn ownership_move() {
    fn takes_ownership(string: String) {
        println!("{}", string)
    }

    let s = String::from("abc");
    takes_ownership(s);
}
//