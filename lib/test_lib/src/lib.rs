#![allow(incomplete_features)]
#![allow(deprecated)]

#![feature(const_generics)]

pub struct MyStruct<const N: usize> {
    _data: [usize; N],
}

impl<const N: usize> MyStruct<N> {
    pub fn new() -> MyStruct<N> {
        return MyStruct {
            _data: unsafe { std::mem::uninitialized() },
        };
    }

    pub fn test_generic(&self) {
        println!("There are {} elements!", N);
    }
}

impl MyStruct<3> {
    pub fn test_three(&self) {
        println!("There are 3 elements! Do something cool with this special case!");
    }
}

