#![feature(const_generics)]

use test_lib::*;

fn main() {
    let mat: Matrix<i32, 3, 3> = unsafe { std::mem::uninitialized() };
    let vec: Vector<i32, 3> = unsafe { std::mem::uninitialized() };
    let res = &mat * &vec;
}
