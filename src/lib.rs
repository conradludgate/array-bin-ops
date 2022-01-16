#![deny(unsafe_op_in_unsafe_fn)]
#![cfg_attr(not(any(doc, test, feature = "std")), no_std)]

use core::ops::{Add, AddAssign, Div, DivAssign, Mul, MulAssign, Sub, SubAssign};

#[cfg(test)]
mod tests;

mod iter;
use iter::{BinOpsIter, Iter};

pub struct Array<T, const N: usize>(pub [T; N]);

fn binop_impl<T, U, O, const N: usize>(
    lhs: [T; N],
    rhs: [U; N],
    op: impl Fn(T, U) -> O + Copy,
) -> [O; N] {
    let mut dc = BinOpsIter::new(lhs, rhs);

    for _ in 0..32 {
        // SAFETY:
        // Will only be called a maximum of N times
        unsafe { dc.step(op) }
    }

    // SAFETY:
    // By this point, we are certain we have initialised all N elements
    unsafe { dc.output() }
}

fn binop_assign_impl<T, U, const N: usize>(
    lhs: &mut [T; N],
    rhs: [U; N],
    op: impl Fn(&mut T, U) + Copy,
) {
    let mut dc = Iter::new(rhs);

    for _ in 0..32 {
        // SAFETY:
        // Will only be called a maximum of N times
        unsafe { op(lhs.get_unchecked_mut(dc.index()), dc.next_unchecked()) }
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
