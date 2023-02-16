use criterion::{black_box, criterion_group, criterion_main, Criterion};
use rand::prelude::*;
use signed::Signed;

fn add_assign<I: Signed>(mut n: I, other: I) -> I {
    let mut abs = n.get_absolute();
    abs += &other;
    n
}

fn benchmark(c: &mut Criterion) {
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

criterion_group!(benches, benchmark);
criterion_main!(benches);
