#![feature(array_zip, array_chunks)]

use std::ops::Add;

use array_bin_ops::Array;
use criterion::{black_box, criterion_group, criterion_main, Criterion};

pub fn criterion_benchmark(c: &mut Criterion) {
    let values: Vec<i32> = (0..512).collect();

    c.bench_function("zip+map32", |b| {
        b.iter(|| {
            black_box(values.array_chunks::<32>().copied().reduce(zip_then_map));
        })
    });

    c.bench_function("zip_map32", |b| {
        b.iter(|| {
            black_box(values.array_chunks::<32>().copied().reduce(zip_map));
        })
    });
}

fn zip_then_map<T: Copy + Add<T, Output = T>, const N: usize>(lhs: [T; N], rhs: [T; N]) -> [T; N] {
    lhs.zip(rhs).map(|(a, b)| a + b)
}

fn zip_map<T: Copy + Add<T, Output = T>, const N: usize>(lhs: [T; N], rhs: [T; N]) -> [T; N] {
    Array(lhs).zip_map(rhs, Add::add)
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
