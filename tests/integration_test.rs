use collections::_test::Rectangle;

/* 集成测试 */

#[test]
fn it_works() {
    assert_eq!(1, 1)
}

#[test]
#[should_panic(expected = "abc")]
fn panic() {

    // expected 是通过contains判断的, 下方的 abcd 包含了 expected 的 abc, 所以会通过
    panic!("abcd");
    // 以下则不通过
    // panic!("xyz");
}

#[test]
fn test_can_hold() {
    let a = Rectangle { x: 10, y: 12 };
    let b = Rectangle { x: 5, y: 5 };
    let c = Rectangle { x: 11, y: 9 };
    assert!(a.can_hold(&b));
    assert!(a.can_hold(&c));
    assert!(!b.can_hold(&a));
}

#[test]
fn test_result() -> Result<(), String> {
    return if true {
        Result::Ok(())
    } else {
        Result::Err("abc".to_string())
    };
}