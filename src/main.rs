#![allow(incomplete_features)]
#![feature(const_generics)]

use test_lib::*;

fn main() {
    let my_struct: MyStruct<3> = MyStruct::new();

    my_struct.test_generic();
    my_struct.test_three();
}
