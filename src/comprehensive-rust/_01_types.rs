fn main() {
    //变量默认是不可变的
    let x = 5;
    println!("The value of x is: {x}");
    x = 6; //❌
    println!("The value of x is: {x}");
    // 可以在变量名前添加 mut 来使其可变
    let mut y = 5;
    y = 6;


    let aint: i64 = 213i64;
    let bint = 2_000;

    let astring = "abc";
    let bstring = r#"\\abc"#;

    let mut achar: char = 'a';
    let mut bchar: char = "a";


}


fn compound_types() {
    let mut a: [i8; 10] = [42; 10];
    a[5] = 0;
    println!("a: {:?}", a);
    println!("a:{:?},{:?}", a, a)
}