use core::mem::needs_drop;
use core::ops::{AddAssign, DivAssign, MulAssign, SubAssign};

use crate::{iter::Slice, Array};

impl<T, const N: usize> Array<T, N> {
    pub fn zip_mut_map<U>(&mut self, rhs: [U; N], op: impl Fn(&mut T, U) + Copy) {
        if needs_drop::<U>() {
            let mut rhs = Slice::full(rhs);

            for i in 0..N {
                // SAFETY:
                // Will only be called a maximum of N times
                unsafe {
                    let lhs = self.0.get_unchecked_mut(i);
                    let rhs = rhs.pop_front_unchecked();
                    op(lhs, rhs)
                }
            }
        } else {
            for i in 0..N {
                // SAFETY:
                // Will only be called a maximum of N times
                unsafe {
                    let lhs = self.0.get_unchecked_mut(i);
                    let rhs = core::ptr::read(&rhs[i]);
                    op(lhs, rhs)
                }
            }
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
                self.zip_mut_map(rhs, T::$method)
            }
        }

        impl<T, U, const N: usize> $trait<Array<U, N>> for Array<T, N>
        where
            T: $trait<U>,
        {
            fn $method(&mut self, rhs: Array<U, N>) {
                self.zip_mut_map(rhs.0, T::$method)
            }
        }
    };
}

binop_assign!(AddAssign, add_assign);
binop_assign!(MulAssign, mul_assign);
binop_assign!(DivAssign, div_assign);
binop_assign!(SubAssign, sub_assign);
