#![allow(
dead_code,
unused_imports,
)]

use crate::learning::{_enum, _generic, _guess_number, _hashmap, _loop, _ownership, _panic, _string, _struct, _vector};

mod learning;

fn main() {
    _enum::main();
    _struct::main();
    _loop::main();
    _guess_number::main();
    _ownership::main();
    _string::main();
    _vector::main();
    _hashmap::main();
    _panic::main();
    _generic::main();
}
