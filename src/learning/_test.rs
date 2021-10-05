#![allow(dead_code)]

pub fn main() {}

pub struct Rectangle {
    pub x: u32,
    pub y: u32,
}

impl Rectangle {
    pub fn can_hold(&self, another: &Rectangle) -> bool {
        return if self.x > another.x && self.y > another.y {
            true
        } else {
            self.x > another.y && self.y > another.x
        };
    }
}

/* 单元测试 */

#[cfg(test)]
mod unit_test {
    use super::*;

    #[test]
    fn test_can_hold() {
        let a = Rectangle { x: 10, y: 12 };
        let b = Rectangle { x: 5, y: 5 };
        let c = Rectangle { x: 11, y: 9 };
        assert!(a.can_hold(&b));
        assert!(a.can_hold(&c));
        assert!(!b.can_hold(&a));
    }
}
