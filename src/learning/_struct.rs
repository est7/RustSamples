#![allow(dead_code)]

pub fn main() {}

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
    }

    let rectangle = Rectangle::new(10, 20);
    println!("area {}", rectangle.area());

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