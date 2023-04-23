#![allow(dead_code)]

use crate::_struct::HEIHEI::He1;
use crate::_struct::HEIHEI::He2;
use crate::_struct::IpAddress::{V1, V3};

pub fn main() {
    struct_print();
    // struct_method_for_compose();
}

fn struct_method() {
    struct Rectangle {
        width: u32,
        height: u32,
    }

    impl Rectangle {
        // 读方法
        fn area(&self) -> u32 {
            self.width * self.height
        }

        // 写方法
        fn set_width(&mut self, width: u32) {
            self.width = width
        }

        // associated functions 关联函数
        fn new(width: u32, height: u32) -> Rectangle {
            Rectangle { width, height }
        }
        // associated functions 关联函数
        fn old(width: u32, height: u32) -> Rectangle {
            Rectangle { width, height }
        }
    }

    let rectangle = Rectangle::new(10, 20);
    let rectangle_old = Rectangle::old(10, 20);
    println!("area {},{}", rectangle.area(), rectangle_old.area());

    let mut rectangle = rectangle;
    rectangle.set_width(15);
    println!("new area {}", rectangle.area());
}

fn struct_print() {
    #[derive(Debug)]
    struct Print {
        number: i32,
        string: String,
        bool: bool,
    }

    let print = Print {
        number: 100,
        string: str("abc"),
        bool: false,
    };

    println!("{:?}", print);
    println!("{:#?}", print);
}

fn struct_demo() {
    // 此处使用无符号整形
    struct Rectangle {
        width: u32,
        height: u32,
    }

    // 此处入参使用借用的形式,避免对象被移动后释放
    fn calculate_area(rectangle: &Rectangle) -> u32 {
        rectangle.width * rectangle.height
    }

    let rectangle = Rectangle { width: 10, height: 20 };

    println!("area: {}", calculate_area(&rectangle))
}

fn struct_tuple() {
    struct Color(i16, i16, i16);
    Color(115, 112, 132);
}

fn struct_create_update() {
    // 定义结构体
    struct User {
        name: String,
        phone: String,
        age: i8,
    }

    fn print_user(user: &User) {
        println!("name:{}, phone:{}, age:{}", user.name, user.phone, user.age);
    }

    // 创建结构体实例
    let mut user = User {
        name: String::from("a"),
        phone: String::from("a"),
        age: 18,
    };


    //区别?
    let a = String::from("a");
    let b = "b";

    // 修改结构体属性
    user.name = String::from("abc");
    print_user(&user);

    // User工厂方法
    fn create_user(
        name: String,
        phone: String,
        age: i8,
    ) -> User {
        User { name, phone, age }
    }

    #[allow(unused)]
        let mut xiao_ming = create_user(
        String::from("小明"),
        String::from("110"),
        18);

    print_user(&xiao_ming);


    // 使用其他结构体更新
    fn update_user(old: User, name: String) -> User {
        User { name, ..old }
    }

    let xiao_hong = create_user(
        str("小红"),
        str("188"),
        19);
    let lzl = update_user(xiao_hong, str("林志玲"));
    print_user(&lzl);
}

fn str(slice: &str) -> String {
    String::from(slice)
}


/*
//❌,这里要声明生命周期
struct User {
    active: bool,
    username: &str,
    email: &str,
    sign_in_count: u64,
}
*/


///struct User 中的泛型参数 'a 表示一个生命周期参数，它的含义是：username 和 email 成员变量的生命周期应该跟随结构体本身（即 'a 所在的作用域）一致。
/// 这样做的好处是，可以确保 User 结构体的引用变量不会指向被释放的内存空间，避免程序运行时出现不可预料的错误
struct User<'a> {
    active: bool,
    username: &'a str,
    email: &'a str,
    sign_in_count: u64,
}


/// 可以使结构体存储被其他对象拥有的数据的引用，不过这么做的话需要用上 生命周期（lifetimes），
/// 生命周期确保结构体引用的数据有效性跟结构体本身保持一致。如果你尝试在结构体中存储一个引用而不指定生命周期将是无效的，比如这样：
fn struct_method_for_compose() {
    let user1 = User {
        active: true,
        username: "someusername123",
        email: "someone@example.com",
        sign_in_count: 1,
    };
}





enum HEIHEI {
    He1,
    He2,
}

enum IpAddress {
    V1,
    V2,
    V3(HEIHEI),
}

fn  if_let_else_method() {
    let address_v1 = V1;
    let address_v2 = IpAddress::V2;
    let address_v1_else = V1;
    let address_v3 = V3(He1);
    match_ip_address(address_v1);
    match_ip_address(address_v3);

    if_let_ip_address(&address_v2);
    if_let_ip_address_and_else(&address_v1_else);
}

fn match_ip_address(address: IpAddress) {
    match address {
        V1 => { println!("{}", "fine_v1") }
        IpAddress::V2 => { println!("{}", "fine_v2") }
        V3(He1) => { println!("{}", "fine_v3_HE_1") }
        V3(He2) => { println!("{}", "fine_v3_HE_2") }
    }
}


fn if_let_ip_address(address: &IpAddress) {
    if let IpAddress::V2 = address {
        println!("{}", "fine_v2")
    }
}

fn if_let_ip_address_and_else(address: &IpAddress) {
    if let IpAddress::V2 = address {
        println!("{}", "fine_v2")
    } else {
        println!("{}", "not_fine_v2")
    }
}

