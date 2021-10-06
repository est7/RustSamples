#![allow(dead_code)]

pub fn main() {}


#[allow(unused_variables)]
fn share_by_box() {
    enum List<V> {
        Next(V, Box<List<V>>),
        Nil,
    }

    let a = List::Next(1, Box::new(
        List::Next(2, Box::new(
            List::Nil))));

    let b = List::Next(3, Box::new(a));
    // let c = Next(4, Box::new(a));
}

#[allow(unused_variables)]
fn share_by_rc() {
    use std::rc::Rc;

    enum List<V> {
        Cons(V, Rc<List<V>>),
        Nil,
    }

    let a = Rc::new(List::Cons(1, Rc::new(List::Nil)));
    {
        println!("a rc {}", Rc::strong_count(&a));

        let b = List::Cons(3, a.clone());
        println!("b rc {}", Rc::strong_count(&a));

        let c = List::Cons(4, Rc::clone(&a));

        println!("c rc {}", Rc::strong_count(&a));
    }

    println!("d rc {}", Rc::strong_count(&a));
}