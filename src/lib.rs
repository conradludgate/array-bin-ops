#![deny(unsafe_op_in_unsafe_fn)]
#![cfg_attr(not(any(doc, test, feature = "std")), no_std)]

use core::ops::{Add, AddAssign, Div, DivAssign, Mul, MulAssign, Sub, SubAssign};

#[cfg(test)]
mod tests;

mod iter;
use iter::Slice;

pub struct Array<T, const N: usize>(pub [T; N]);

fn binop_impl<T, U, O, const N: usize>(
    lhs: [T; N],
    rhs: [U; N],
    op: impl Fn(T, U) -> O + Copy,
) -> [O; N] {
    let mut lhs = Slice::full(lhs);
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
}

fn zip<T, U, const N: usize>(
    lhs: [T; N],
    rhs: [U; N],
) -> [(T, U); N] {
    let mut lhs = Slice::full(lhs);
    let mut rhs = Slice::full(rhs);
    let mut output = Slice::new();

    for _ in 0..N {
        unsafe {
            let lhs = lhs.pop_front_unchecked();
            let rhs = rhs.pop_front_unchecked();
            output.push_unchecked((lhs, rhs));
        }
    }

    unsafe { output.output() }
}

fn binop_assign_impl<T, U, const N: usize>(
    lhs: &mut [T; N],
    rhs: [U; N],
    op: impl Fn(&mut T, U) + Copy,
) {
    let mut rhs = Slice::full(rhs);

    for i in 0..N {
        // SAFETY:
        // Will only be called a maximum of N times
        unsafe { op(lhs.get_unchecked_mut(i), rhs.pop_front_unchecked()) }
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
                binop_impl(self.0, rhs, T::$method)
            }
        }

        impl<T, U, const N: usize> $trait<Array<U, N>> for Array<T, N>
        where
            T: $trait<U>,
        {
            type Output = Array<T::Output, N>;

            fn $method(self, rhs: Array<U, N>) -> Self::Output {
                Array(binop_impl(self.0, rhs.0, T::$method))
            }
        }
    };
}

macro_rules! binop_assign {
    ($trait:ident, $method:ident) => {
        impl<T, U, const N: usize> $trait<[U; N]> for Array<T, N>
        where
            T: $trait<U>,
        {
            fn $method(&mut self, rhs: [U; N]) {
                binop_assign_impl(&mut self.0, rhs, T::$method)
            }
        }

        impl<T, U, const N: usize> $trait<Array<U, N>> for Array<T, N>
        where
            T: $trait<U>,
        {
            fn $method(&mut self, rhs: Array<U, N>) {
                binop_assign_impl(&mut self.0, rhs.0, T::$method)
            }
        }
    };
}

binop!(Add, add);
binop!(Mul, mul);
binop!(Div, div);
binop!(Sub, sub);

binop_assign!(AddAssign, add_assign);
binop_assign!(MulAssign, mul_assign);
binop_assign!(DivAssign, div_assign);
binop_assign!(SubAssign, sub_assign);
