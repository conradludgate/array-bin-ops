#![feature(array_zip, array_chunks)]

use std::ops::Add;

use array_bin_ops::Array;
use criterion::{black_box, criterion_group, criterion_main, Criterion};

pub fn criterion_benchmark(c: &mut Criterion) {
    let values: Vec<i32> = (0..5120).collect();

    c.bench_function("std_map128", |b| {
        b.iter(|| {
            black_box(values.array_chunks::<128>().copied().reduce(std_map));
        })
    });

    c.bench_function("our_map128", |b| {
        b.iter(|| {
            black_box(values.array_chunks::<128>().copied().reduce(our_map));
        })
    });

    c.bench_function("std_from_fn128", |b| {
        b.iter(|| {
            black_box(values.array_chunks::<128>().copied().reduce(std_from_fn));
        })
    });

    c.bench_function("our_from_fn128", |b| {
        b.iter(|| {
            black_box(values.array_chunks::<128>().copied().reduce(our_from_fn));
        })
    });

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

fn std_map<T: Copy + Add<T, Output = T>, const N: usize>(lhs: [T; N], rhs: [T; N]) -> [T; N] {
    zip_map(lhs.map(|a| a + a), rhs.map(|a| a + a))
}

fn our_map<T: Copy + Add<T, Output = T>, const N: usize>(lhs: [T; N], rhs: [T; N]) -> [T; N] {
    zip_map(Array(lhs).map(|a| a + a), rhs.map(|a| a + a))
}

fn std_from_fn<const N: usize>(lhs: [i32; N], rhs: [i32; N]) -> [i32; N] {
    let x = zip_map(lhs, rhs);
    let y = std::array::from_fn(|idx| idx as i32);
    zip_map(x, y)
}

fn our_from_fn<const N: usize>(lhs: [i32; N], rhs: [i32; N]) -> [i32; N] {
    let x = zip_map(lhs, rhs);
    let mut idx = 0;
    let y = Array([(); N]).map(|_| {
        let res = idx as i32;
        idx += 1;
        res
    });
    zip_map(x, y)
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
