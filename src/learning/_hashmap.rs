use std::collections::HashMap;

pub fn main() {}

fn update() {
    // 覆盖
    let mut map = HashMap::new();
    map.insert("a".to_string(), 10);
    map.insert("a".to_string(), 20);
    println!("{:?}", map);

    // 没有值时插入,有值时忽略
    let mut map = HashMap::new();
    map.insert("a".to_string(), 10);
    map.entry("a".to_string()).or_insert(20);
    println!("{:?}", map);

    // 借用值的可修改应用,根据需要修改值
    let mut map = HashMap::new();
    // map.insert("a".to_string(), 10);
    let value = map.entry("a".to_string()).or_insert(10);
    *value = 20;
    println!("{:?}", map);
}

fn collect() {
    let a = vec!["a".to_string(), "b".to_string()];
    let b = vec![1];
    let map: HashMap<&String, i32> = a.iter().zip(b).collect();
    println!("{:?}", map);
}