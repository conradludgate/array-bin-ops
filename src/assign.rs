use core::mem::needs_drop;
use core::ops::{AddAssign, DivAssign, MulAssign, SubAssign};

use crate::{iter::Slice, Array};

fn binop_assign_impl<T, U, const N: usize>(
    lhs: &mut [T; N],
    rhs: [U; N],
    op: impl Fn(&mut T, U) + Copy,
) {
    if !needs_drop::<U>() {
        // SAFETY:
        // we've just checked that U is a non-drop type
        unsafe { binop_assign_impl_copy(lhs, rhs, op) }
    } else {
        binop_assign_impl_drop(lhs, rhs, op)
    }
}

fn binop_assign_impl_drop<T, U, const N: usize>(
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

/// # Safety
/// must only be called if U is a copy type (no drop needed)
unsafe fn binop_assign_impl_copy<T, U, const N: usize>(
    lhs: &mut [T; N],
    rhs: [U; N],
    op: impl Fn(&mut T, U) + Copy,
) {
    for i in 0..N {
        // SAFETY:
        // Will only be called a maximum of N times
        unsafe {
            let rhs = core::ptr::read(&rhs[i]);
            op(lhs.get_unchecked_mut(i), rhs)
        }
    }
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

binop_assign!(AddAssign, add_assign);
binop_assign!(MulAssign, mul_assign);
binop_assign!(DivAssign, div_assign);
binop_assign!(SubAssign, sub_assign);
