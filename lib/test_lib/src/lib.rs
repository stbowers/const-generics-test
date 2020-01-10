#![feature(const_generics)]

use std::ops::{Add, Mul};

pub struct Matrix<T, const M: usize, const N: usize> {
    data: Vector<T, {N*M}>,
}

pub struct Vector<T, const N: usize> {
    data: [T; N],
}

impl<T: Copy + Add<Output = T> + Mul<Output = T>> Mul<&Vector<T, 3>> for &Matrix<T, 3, 3> {
    type Output = Vector<T, 3>;

    fn mul(self, rhs: & Vector<T, 3>) -> Vector<T, 3> {
        // The actual calculation doesn't matter
        return unsafe { std::mem::uninitialized() }
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
