pub fn main() {}

fn modify_when_iter() {
    let mut vec = vec![1, 2, 3];

    // 利用可修改遍历方法
    for ele in vec.iter_mut() {
        *ele += 1
    }
    println!("way 1: {:?}", vec);

    // 利用长度遍历
    for index in 0..vec.len() {
        if let Some(ele) = vec.get_mut(index) {
            *ele += 1;
        }
    }
    println!("way 2: {:?}", vec);

    // 利用map转换为另一个vector
    let vec: Vec<i32> = vec.iter()
        .map(|ele| ele + 1)
        .collect();
    println!("way 3: {:?}", vec);
}

fn modify() {
    let mut vec = vec![1, 2, 3];
    vec[1] = 10;

    for ele in vec {
        println!("{}", ele)
    }
}

fn push_multi_type() {
    enum Value {
        Float(f64),
        Int(i32),
        String(String),
    }

    let v = vec![
        Value::Float(1.2),
        Value::Int(1),
        Value::String(String::from("abc")),
    ];

    for value in v {
        match value {
            Value::Int(value) => {
                println!("int {}", value)
            }
            Value::Float(value) => {
                println!("float {}", value)
            }
            Value::String(value) => {
                println!("string {}", value)
            }
        }
    }
}

fn vector_iterator() {
    let mut v = Vec::new();
    v.push(String::from("abc"));
    v.push(String::from("cba"));

    for str in &v {
        println!("{}", str);
    }
}

fn vector_ownership() {
    let mut v = Vec::new();

    v.push(String::from("abc"));
    let element = &v[0];
    println!("{}", element);

    // 这里借用了一次可修改指针, 导致v被锁定
    v.push(String::from("cba"));

    // 下方尝试获取slice的不可修改指针, 会导致数据竞争, 编译失败
    // println!("{}", element);

    // 如果需要重新读取, 可以shadow掉旧的element
    let element = &v[0];
    println!("{}", element);
}


fn vector_get() {
    let mut v_i32 = Vec::new();
    v_i32.push(1);
    v_i32.push(2);

    // 通过索引获取,越界会引发崩溃
    let element_0 = &v_i32[0];
    println!("{}", element_0);

    // 通过get方法,越界会返回None
    if let Some(value) = v_i32.get(2) {
        println!("{}", value);
    } else {
        println!("!!!");
    }
}