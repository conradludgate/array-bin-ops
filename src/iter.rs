use core::mem::transmute_copy;
use core::mem::{ManuallyDrop, MaybeUninit};
use core::ptr::drop_in_place;
use std::ops::Range;

pub struct Slice<T, const N: usize> {
    array: [MaybeUninit<T>; N],
    alive: Range<usize>,
}

impl<T, const N: usize> Drop for Slice<T, N> {
    #[inline(always)]
    fn drop(&mut self) {
        // SAFETY:
        // The slice guarantees that `alive` is all initialised and that the other data is uninit
        unsafe {
            drop_in_place(slice_assume_init_mut(&mut self.array[self.alive.clone()]));
        }
    }
}

impl<T, const N: usize> Slice<T, N> {
    #[inline(always)]
    pub fn new() -> Self {
        Self {
            array: uninit_array(),
            alive: 0..0,
        }
    }

    #[inline(always)]
    pub fn full(arr: [T; N]) -> Self {
        Self {
            array: mu_array(arr),
            alive: 0..N,
        }
    }

    /// # Safety
    /// All values of output must be initialised, and all values in the inputs must be consumed
    #[inline(always)]
    pub unsafe fn output(self) -> [T; N] {
        debug_assert_eq!(self.alive, 0..N);

        let md = ManuallyDrop::new(self);
        // SAFETY:
        // caller is responsible for ensuring the output is fully initialised
        unsafe { assume_array_init(&md.array) }
    }

    #[inline(always)]
    pub unsafe fn pop_front_unchecked(&mut self) -> T {
        debug_assert!(!self.alive.is_empty());
        debug_assert!(self.alive.start < N);

        unsafe {
            let front = take(self.array.get_unchecked_mut(self.alive.start));
            self.alive.start += 1;
            front
        }
    }

    #[inline(always)]
    pub unsafe fn push_unchecked(&mut self, value: T) {
        debug_assert!(self.alive.end < N);

        unsafe {
            self.array.get_unchecked_mut(self.alive.end).write(value);
            self.alive.end += 1;
        }
    }
}

#[inline(always)]
pub unsafe fn take<T>(slot: &mut MaybeUninit<T>) -> T {
    // SAFETY: we are reading from a reference, which is guaranteed
    // to be valid for reads.
    unsafe { core::ptr::read(slot.assume_init_mut()) }
}

/// Create a new `[ManuallyDrop<U>; N]` from the initialised array
#[inline(always)]
fn mu_array<T, const N: usize>(a: [T; N]) -> [MaybeUninit<T>; N] {
    a.map(MaybeUninit::new)
}

#[inline(always)]
pub fn uninit_array<T, const N: usize>() -> [MaybeUninit<T>; N] {
    // SAFETY: An uninitialized `[MaybeUninit<_>; N]` is valid.
    // replace with [`MaybeUninit::uninit_array`] in std when stable
    unsafe { MaybeUninit::<[MaybeUninit<T>; N]>::uninit().assume_init() }
}

#[inline(always)]
pub unsafe fn assume_array_init<T, const N: usize>(a: &[MaybeUninit<T>; N]) -> [T; N] {
    // SAFETY: MaybeUninit is guaranteed to have the same layout
    // replace with [`MaybeUninit::array_assume_init`] in std when stable
    unsafe { transmute_copy(a) }
}

#[inline(always)]
pub unsafe fn slice_assume_init_mut<T>(slice: &mut [MaybeUninit<T>]) -> &mut [T] {
    // SAFETY: similar to safety notes for `slice_get_ref`, but we have a
    // mutable reference which is also guaranteed to be valid for writes.
    unsafe { &mut *(slice as *mut [MaybeUninit<T>] as *mut [T]) }
}
