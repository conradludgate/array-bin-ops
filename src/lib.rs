#![deny(unsafe_op_in_unsafe_fn)]
#![cfg_attr(not(any(doc, test, feature = "std")), no_std)]

#[cfg(test)]
mod tests;

pub mod assign;
pub mod simple;
mod iter;

#[repr(transparent)]
pub struct Array<T, const N: usize>(pub [T; N]);

impl<T, const N: usize> Array<T, N> {
    pub fn new_mut(a: &mut [T; N]) -> &mut Self {
        // SAFETY:
        // representation is the same
        unsafe { &mut *(a.as_mut_ptr().cast()) }
    }
}
