use crate::Array;

use mockalloc::Mockalloc;
use std::{alloc::System, ops::Add, panic::resume_unwind};

#[global_allocator]
static ALLOCATOR: Mockalloc<System> = Mockalloc(System);

#[test]
fn add() {
    let a: [i32; 4] = [0, 1, 2, 3];
    let b: [i32; 4] = [4, 5, 6, 7];

    assert_eq!(Array(a) + b, [4, 6, 8, 10]);
}

#[test]
fn add_assign() {
    let a: [i32; 4] = [0, 1, 2, 3];
    let b: [i32; 4] = [4, 5, 6, 7];

    let mut a = Array(a);

    a += b;

    assert_eq!(a.0, [4, 6, 8, 10]);
}

#[mockalloc::test]
fn no_panic_no_double_free() {
    let a = [0, 1, 2, 3].map(HeapAdd::new);
    let b = [4, 5, 6, 7].map(HeapAdd::new);

    let exp = [4, 6, 8, 10].map(HeapAdd::new);

    assert_eq!(Array(a) + b, exp);
}

#[mockalloc::test]
fn panic_no_double_free() {
    let a = [0, 1, 2, 3].map(HeapAdd::new);
    let b = [6, 7, 8, 9].map(HeapAdd::new);

    std::panic::catch_unwind(|| Array(a) + b)
        .expect_err("array add is intended to panic in this test");
}

#[derive(PartialEq, Debug)]
struct HeapAdd(Box<i32>);
impl HeapAdd {
    fn new(x: i32) -> Self {
        Self(Box::new(x))
    }
}

impl Add for HeapAdd {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        if *rhs.0 == 8 {
            resume_unwind(Box::new("adding 8 is not allowed"));
        }

        HeapAdd(Box::new(*self.0 + *rhs.0))
    }
}
