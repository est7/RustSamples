#![allow(dead_code)]

fn main() {
    loop_expression();
}

fn loop_expression() {
    let mut counter = 0;

    let result = loop {
        counter += 1;

        if counter == 10 {
            break counter * 2;
        }
    };

    println!("The result is {result}");
}

///外层循环有一个标签 counting_up，它将从 0 数到 2。没有标签的内部循环从 10 向下数到 9。
/// 第一个没有指定标签的 break 将只退出内层循环。break 'counting_up; 语句将退出外层循环。
fn nested_loop_expression() {
    let mut count = 0;
    'counting_up: loop {
        println!("count = {count}");
        let mut remaining = 10;

        loop {
            println!("remaining = {remaining}");
            if remaining == 9 {
                break;
            }
            if count == 2 {
                break 'counting_up;
            }
            remaining -= 1;
        }

        count += 1;
    }
    println!("End count = {count}");
}

fn while_expression() {
    let mut number = 3;

    while number != 0 {
        println!("{number}!");

        number -= 1;
    }

    println!("LIFTOFF!!!");
}


fn for_expression() {
    let a = [10, 20, 30, 40, 50];

    for element in a {
        println!("the value is: {element}");
    }

    for number in (1..20).rev() {
        println!("{number}!");
    }
    println!("LIFTOFF!!!");
}


fn control() {
    let number = 6;

    if number % 4 == 0 {
        println!("number is divisible by 4");
    } else if number % 3 == 0 {
        println!("number is divisible by 3");
    } else if number % 2 == 0 {
        println!("number is divisible by 2");
    } else {
        println!("number is not divisible by 4, 3, or 2");
    }

    let condition = true;
    let number = if condition { 5 } else { 6 };

    println!("The value of number is: {}", number);

    // rust 是强类型语言，必须在编译时候确变量类型
    // 会报错，类型不一致
    //❌
    //let number = if condition { 5 } else { "six" };

    let mut count = 0;
    'counting_up: loop {
        println!("count = {}", count);
        let mut remaining = 10;

        loop {
            println!("remaining = {}", remaining);
            if remaining == 9 {
                break;
            }
            if count == 2 {
                break 'counting_up;
            }
            remaining -= 1;
        }

        count += 1;
    }


    //loop
    let mut counter = 0;

    let result = loop { // 从循环返回
        counter += 1;

        if counter == 10 {
            // break counter * 2;
            break counter * 2;
        } else {
            "fad";
        };
    };


    // while
    let mut number = 3;

    while number != 0 {
        println!("{}!", number);

        number -= 1;
    }


    // for
    let a = [10, 20, 30, 40, 50];

    for element in a {
        println!("the value is: {}", element);
    }

    for number in (1..4).rev() {
        println!("{}!", number);
    }
}

fn if_expression() {
    let number = 3;

    if number < 5 {
        println!("condition was true");
    } else {
        println!("condition was false");
    }


    let str = "fas";

    let a = if number == 5 && str == "fas" {
        println!("condition was true");
        "dfa"
    } else {
        println!("condition was false");
        "fasdf"
    };
}