#![deny(unsafe_op_in_unsafe_fn)]
#![cfg_attr(not(any(doc, test, feature = "std")), no_std)]

#[cfg(test)]
mod tests;

pub mod assign;
pub mod simple;
mod iter;

pub struct Array<T, const N: usize>(pub [T; N]);
