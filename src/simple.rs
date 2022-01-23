use core::mem::{needs_drop, MaybeUninit};
use core::ops::{Add, Div, Mul, Sub};

use crate::iter::uninit_array;
use crate::{iter::Slice, Array};

fn binop_impl<T, U, O, const N: usize>(
    lhs: [T; N],
    rhs: [U; N],
    op: impl Fn(T, U) -> O + Copy,
) -> [O; N] {
    if !needs_drop::<T>() && !needs_drop::<U>() && !needs_drop::<O>() {
        // SAFETY:
        // we've just checked that T, U and O are non-drop types
        unsafe { binop_impl_copy(lhs, rhs, op) }
    } else {
        binop_impl_drop(lhs, rhs, op)
    }
}

fn binop_impl_drop<T, U, O, const N: usize>(
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

/// # Safety
/// must only be called if T, U and O are Copy types (no drop needed)
unsafe fn binop_impl_copy<T, U, O, const N: usize>(
    lhs: [T; N],
    rhs: [U; N],
    op: impl Fn(T, U) -> O + Copy,
) -> [O; N] {
    // SAFETY:
    // we will not read from output, and caller ensures that O is non-drop
    let mut output: [MaybeUninit<O>; N] = uninit_array();

    for i in 0..N {
        unsafe {
            let lhs = core::ptr::read(&lhs[i]);
            let rhs = core::ptr::read(&rhs[i]);
            output[i].write(op(lhs, rhs));
        }
    }

    unsafe { core::ptr::read(&output as *const [MaybeUninit<O>; N] as *const [O; N]) }
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

binop!(Add, add);
binop!(Mul, mul);
binop!(Div, div);
binop!(Sub, sub);
