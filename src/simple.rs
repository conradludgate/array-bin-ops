use core::mem::{needs_drop, MaybeUninit};
use core::ops::{Add, Div, Mul, Sub};

use crate::iter::uninit_array;
use crate::{iter::Slice, Array};

impl<T, const N: usize> Array<T, N> {
    pub fn zip_map<U, O>(self, rhs: [U; N], mut op: impl FnMut(T, U) -> O) -> [O; N] {
        if needs_drop::<T>() || needs_drop::<U>() || needs_drop::<O>() {
            let mut lhs = Slice::full(self.0);
            let mut rhs = Slice::full(rhs);
            let mut output = Slice::new();

            for _ in 0..N {
                unsafe {
                    let lhs = lhs.pop_front_unchecked();
                    let rhs = rhs.pop_front_unchecked();
                    output.push_unchecked(op(lhs, rhs));
                }
            }

            unsafe { output.output() }
        } else {
            // SAFETY:
            // we will not read from output, and caller ensures that O is non-drop
            let mut output: [MaybeUninit<O>; N] = uninit_array();

            for i in 0..N {
                unsafe {
                    let lhs = core::ptr::read(&self.0[i]);
                    let rhs = core::ptr::read(&rhs[i]);
                    output[i].write(op(lhs, rhs));
                }
            }

            unsafe { core::ptr::read(&output as *const [MaybeUninit<O>; N] as *const [O; N]) }
        }
    }
}

macro_rules! binop {
    ($trait:ident, $method:ident) => {
        impl<T, U, const N: usize> $trait<[U; N]> for Array<T, N>
        where
            T: $trait<U>,
        {
            type Output = [T::Output; N];

            fn $method(self, rhs: [U; N]) -> Self::Output {
                self.zip_map(rhs, T::$method)
            }
        }

        impl<T, U, const N: usize> $trait<Array<U, N>> for Array<T, N>
        where
            T: $trait<U>,
        {
            type Output = Array<T::Output, N>;

            fn $method(self, rhs: Array<U, N>) -> Self::Output {
                Array(self.zip_map(rhs.0, T::$method))
            }
        }
    };
}

binop!(Add, add);
binop!(Mul, mul);
binop!(Div, div);
binop!(Sub, sub);
