use core::mem::{ManuallyDrop, MaybeUninit};
use core::ptr::drop_in_place;
use core::mem::transmute_copy;

/// Like [`Iter`], but traverses 3 arrays at once
pub struct BinOpsIter<T, U, O, const N: usize> {
    rhs: Iter<U, N>,
    lhs: [ManuallyDrop<T>; N],
    output: [MaybeUninit<O>; N],
}

impl<T, U, O, const N: usize> Drop for BinOpsIter<T, U, O, N> {
    fn drop(&mut self) {
        let i = self.rhs.i;
        // SAFETY:
        // `i` defines how many elements have been processed from the arrays.
        // Caveat, the only potential panic would happen *before* the write to the output,
        // so the `i`th output is not initialised as one would assume.
        unsafe {
            drop_in_place((&mut self.lhs[i..]) as *mut [_] as *mut [T]);
            drop_in_place(&mut self.output[..i - 1] as *mut [_] as *mut [O]);
        }
    }
}

impl<T, U, O, const N: usize> BinOpsIter<T, U, O, N> {
    pub fn new(lhs: [T; N], rhs: [U; N]) -> Self {
        Self {
            rhs: Iter::new(rhs),
            lhs: md_array(lhs),
            output: uninit_array(),
        }
    }

    /// # Safety
    /// All values of output must be initialised, and all values in the inputs must be consumed
    pub unsafe fn output(self) -> [O; N] {
        debug_assert_eq!(self.rhs.i, N);

        let md = ManuallyDrop::new(self);
        // SAFETY:
        // caller is responsible for ensuring the output is fully initialised
        unsafe { assume_array_init(&md.output) }
    }

    /// # Safety
    /// Must be called no more than `N` times.
    pub unsafe fn step(&mut self, f: impl FnOnce(T, U) -> O) {
        // SAFETY:
        // Since `dc.i` is stricty-monotonic, we will only
        // take each element only once from each of lhs/rhs
        unsafe {
            let i = self.rhs.i;
            let rhs = self.rhs.next_unchecked();
            let lhs = ManuallyDrop::take(self.lhs.get_unchecked_mut(i));
            let out = self.output.get_unchecked_mut(i);
            out.write(f(lhs, rhs));
        }
    }
}

/// For sake of optimisation, it's a simplified version of [`array::IntoIter`]
/// that can only go forward, and can only be accessed through unsafe (to avoid bounds checks)
pub struct Iter<U, const N: usize> {
    rhs: [ManuallyDrop<U>; N],
    i: usize,
}

impl<U, const N: usize> Drop for Iter<U, N> {
    fn drop(&mut self) {
        let i = self.i;
        // SAFETY:
        // `i` defines how many elements have been processed from the array,
        // meaning that theres `i..` elements left to process (and therefore, drop)
        unsafe {
            drop_in_place((&mut self.rhs[i..]) as *mut [_] as *mut [U]);
        }
    }
}

impl<U, const N: usize> Iter<U, N> {
    pub fn new(rhs: [U; N]) -> Self {
        Self {
            rhs: md_array(rhs),
            i: 0,
        }
    }

    /// # Safety
    /// Must be called no more than `N` times.
    #[inline(always)]
    pub unsafe fn next_unchecked(&mut self) -> U {
        debug_assert!(self.i < N);

        // SAFETY:
        // Caller ensures that next is not called more than `N` times, so self.i must be
        // smaller than N at this point
        let rhs = unsafe { self.rhs.get_unchecked_mut(self.i) };

        // SAFETY:
        // Since `dc.i` is stricty-monotonic, we will only
        // take each element only once from each of lhs/rhs
        let rhs = unsafe { ManuallyDrop::take(rhs) };

        self.i += 1;
        rhs
    }
}

/// Create a new `[ManuallyDrop<U>; N]` from the initialised array
fn md_array<T, const N: usize>(a: [T; N]) -> [ManuallyDrop<T>; N] {
    a.map(ManuallyDrop::new)
}

pub fn uninit_array<T, const N: usize>() -> [MaybeUninit<T>; N] {
    // SAFETY: An uninitialized `[MaybeUninit<_>; N]` is valid.
    // replace with [`MaybeUninit::uninit_array`] in std when stable
    unsafe { MaybeUninit::<[MaybeUninit<T>; N]>::uninit().assume_init() }
}

pub unsafe fn assume_array_init<T, const N: usize>(a: &[MaybeUninit<T>; N]) -> [T; N] {
    // SAFETY: MaybeUninit is guaranteed to have the same layout
    // replace with [`MaybeUninit::assume_array_init`] in std when stable
    unsafe { transmute_copy(a) }
}
