use criterion::{black_box, criterion_group, criterion_main, BatchSize, BenchmarkId, Criterion};
use str_manip_bench::{fmt, mutate_inplace, with_cap};

pub fn short(c: &mut Criterion) {
    let s = String::from("asdf");
    c.bench_with_input(BenchmarkId::new("short_fmt", "asdf"), &s, |b, s| {
        b.iter_batched(
            || s.clone(),
            |inp| fmt(black_box(inp)),
            BatchSize::SmallInput,
        );
    });
    c.bench_with_input(
        BenchmarkId::new("short_mutatie_inplace", "asdf"),
        &s,
        |b, s| {
            b.iter_batched(
                || s.clone(),
                |mut inp| mutate_inplace(black_box(&mut inp)),
                BatchSize::SmallInput,
            );
        },
    );
    c.bench_with_input(BenchmarkId::new("short_with_cap", "asdf"), &s, |b, s| {
        b.iter_batched(
            || s.clone(),
            |inp| with_cap(black_box(inp)),
            BatchSize::SmallInput,
        );
    });
}
pub fn long(c: &mut Criterion) {
    let s = String::from(include_str!("lorem.txt"));

    c.bench_with_input(BenchmarkId::new("logn_fmt", "lorum"), &s, |b, s| {
        b.iter_batched(
            || s.clone(),
            |inp| fmt(black_box(inp)),
            BatchSize::SmallInput,
        );
    });
    c.bench_with_input(
        BenchmarkId::new("logn_mutatie_inplace", "lorum"),
        &s,
        |b, s| {
            b.iter_batched(
                || s.clone(),
                |mut inp| mutate_inplace(black_box(&mut inp)),
                BatchSize::SmallInput,
            );
        },
    );
    c.bench_with_input(BenchmarkId::new("logn_with_cap", "lorum"), &s, |b, s| {
        b.iter_batched(
            || s.clone(),
            |inp| with_cap(black_box(inp)),
            BatchSize::SmallInput,
        );
    });
}

criterion_group! {
name = bench_short;
config = Criterion::default().sample_size(4000);
targets = short
}
criterion_group! {
name = bench_long;
config = Criterion::default().sample_size(4000);
targets = long
}
criterion_main!(bench_short, bench_long);
