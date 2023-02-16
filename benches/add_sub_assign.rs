use criterion::{black_box, criterion_group, criterion_main, Criterion};
use rand::prelude::*;
use signed::Signed;

fn add_assign<I: Signed>(mut n: I, other: I) -> I {
    let mut abs = n.get_absolute();
    abs += &other;
    n
}

fn sub_assign<I: Signed>(mut n: I, other: I) -> I {
    let mut abs = n.get_absolute();
    abs -= &other;
    n
}

fn benchmark_add_i8(c: &mut Criterion) {
    let mut rng = thread_rng();
    c.bench_function("add_assign_i8", |b| {
        b.iter_batched(
            || {
                let n: i8 = rng.gen();
                let other: i8 = rng.gen();
                (n, other)
            },
            |(n, other)| add_assign(black_box(n), black_box(other)),
            criterion::BatchSize::SmallInput,
        )
    });
}

fn benchmark_add_i16(c: &mut Criterion) {
    let mut rng = thread_rng();
    c.bench_function("add_assign_i16", |b| {
        b.iter_batched(
            || {
                let n: i16 = rng.gen();
                let other: i16 = rng.gen();
                (n, other)
            },
            |(n, other)| add_assign(black_box(n), black_box(other)),
            criterion::BatchSize::SmallInput,
        )
    });
}

fn benchmark_add_i32(c: &mut Criterion) {
    let mut rng = thread_rng();
    c.bench_function("add_assign_i32", |b| {
        b.iter_batched(
            || {
                let n: i32 = rng.gen();
                let other: i32 = rng.gen();
                (n, other)
            },
            |(n, other)| add_assign(black_box(n), black_box(other)),
            criterion::BatchSize::SmallInput,
        )
    });
}

fn benchmark_add_i64(c: &mut Criterion) {
    let mut rng = thread_rng();
    c.bench_function("add_assign_i64", |b| {
        b.iter_batched(
            || {
                let n: i64 = rng.gen();
                let other: i64 = rng.gen();
                (n, other)
            },
            |(n, other)| add_assign(black_box(n), black_box(other)),
            criterion::BatchSize::SmallInput,
        )
    });
}

fn benchmark_add_i128(c: &mut Criterion) {
    let mut rng = thread_rng();
    c.bench_function("add_assign_i128", |b| {
        b.iter_batched(
            || {
                let n: i128 = rng.gen();
                let other: i128 = rng.gen();
                (n, other)
            },
            |(n, other)| add_assign(black_box(n), black_box(other)),
            criterion::BatchSize::SmallInput,
        )
    });
}

fn benchmark_sub_i8(c: &mut Criterion) {
    let mut rng = thread_rng();
    c.bench_function("sub_assign_i8", |b| {
        b.iter_batched(
            || {
                let n: i8 = rng.gen();
                let other: i8 = rng.gen();
                (n, other)
            },
            |(n, other)| sub_assign(black_box(n), black_box(other)),
            criterion::BatchSize::SmallInput,
        )
    });
}

fn benchmark_sub_i16(c: &mut Criterion) {
    let mut rng = thread_rng();
    c.bench_function("sub_assign_i16", |b| {
        b.iter_batched(
            || {
                let n: i16 = rng.gen();
                let other: i16 = rng.gen();
                (n, other)
            },
            |(n, other)| sub_assign(black_box(n), black_box(other)),
            criterion::BatchSize::SmallInput,
        )
    });
}

fn benchmark_sub_i32(c: &mut Criterion) {
    let mut rng = thread_rng();
    c.bench_function("sub_assign_i32", |b| {
        b.iter_batched(
            || {
                let n: i32 = rng.gen();
                let other: i32 = rng.gen();
                (n, other)
            },
            |(n, other)| sub_assign(black_box(n), black_box(other)),
            criterion::BatchSize::SmallInput,
        )
    });
}

fn benchmark_sub_i64(c: &mut Criterion) {
    let mut rng = thread_rng();
    c.bench_function("sub_assign_i64", |b| {
        b.iter_batched(
            || {
                let n: i64 = rng.gen();
                let other: i64 = rng.gen();
                (n, other)
            },
            |(n, other)| sub_assign(black_box(n), black_box(other)),
            criterion::BatchSize::SmallInput,
        )
    });
}

fn benchmark_sub_i128(c: &mut Criterion) {
    let mut rng = thread_rng();
    c.bench_function("sub_assign_i128", |b| {
        b.iter_batched(
            || {
                let n: i128 = rng.gen();
                let other: i128 = rng.gen();
                (n, other)
            },
            |(n, other)| sub_assign(black_box(n), black_box(other)),
            criterion::BatchSize::SmallInput,
        )
    });
}

criterion_group!(
    benches_add,
    benchmark_add_i8,
    benchmark_add_i16,
    benchmark_add_i32,
    benchmark_add_i64,
    benchmark_add_i128
);
criterion_group!(
    benches_sub,
    benchmark_sub_i8,
    benchmark_sub_i16,
    benchmark_sub_i32,
    benchmark_sub_i64,
    benchmark_sub_i128
);
criterion_main!(benches_add, benches_sub);
