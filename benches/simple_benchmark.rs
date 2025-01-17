/* std use */

/* crates use */
use criterion::{black_box, criterion_group, criterion_main, BenchmarkId, Criterion};
use random_string::generate;

/* project use */
use kmers::naive_impl;

use kmers::encoding::Encoding as _;

const K: usize = 31;

pub fn compute_naive(b: &[u8]) -> u64 {
    b.windows(K)
        .map(|x| {
            let k = naive_impl::Kmer::from(x);

            k.into_u64()
        })
        .sum()
}

pub fn compute_xor10(b: &[u8]) -> u64 {
    let enc = kmers::encoding::xor10::Xor10;

    b.windows(K)
        .map(|x| {
            let k =
                kmers::kmer::Kmer::<u64, K, { kmers::kmer::word_for_k::<u64, K>() }>::new(x, &enc);
            k.num_bytes() as u64
        })
        .sum()
}

pub fn rc_naive_prepare(b: &[u8]) -> Vec<naive_impl::Kmer> {
    b.windows(K).map(naive_impl::Kmer::from).collect()
}

pub fn rc_naive(b: &[naive_impl::Kmer]) -> u64 {
    b.iter()
        .map(|k| {
            k.to_reverse_complement();
            k.into_u64()
        })
        .sum::<u64>()
}

pub fn rc_xor10_prepare(b: &[u8]) -> Vec<[u64; 1]> {
    let enc = kmers::encoding::xor10::Xor10;

    b.windows(K).map(|x| enc.encode(x)).collect()
}

pub fn rc_xor10(b: &[[u64; 1]]) -> u64 {
    let enc = kmers::encoding::xor10::Xor10;

    b.iter()
        .map(|k| {
            enc.rev_comp::<K>(*k);
            k[0]
        })
        .sum::<u64>()
}

pub fn construct(c: &mut Criterion) {
    let charset = "ACGT";

    let mut g = c.benchmark_group("construct");

    for i in 8..16 {
        let input = generate(1 << i, charset);
        let bytes = input.as_bytes();

        g.bench_with_input(BenchmarkId::new("naive", 1 << i), &bytes, |b, &s| {
            b.iter(|| black_box(compute_naive(s)));
        });

        g.bench_with_input(
            BenchmarkId::new("kme.rs::xor10", 1 << i),
            &bytes,
            |b, &s| {
                b.iter(|| black_box(compute_xor10(s)));
            },
        );
    }
}

pub fn reverse_complement(c: &mut Criterion) {
    let charset = "ACGT";

    let mut g = c.benchmark_group("reverse_complement");

    for i in 8..16 {
        let input = generate(1 << i, charset);
        let bytes = input.as_bytes();

        let naive_kmers = rc_naive_prepare(bytes);
        let xor10_kmers = rc_xor10_prepare(bytes);

        g.bench_with_input(
            BenchmarkId::new("naive", 1 << i),
            &naive_kmers,
            |b, kmers| {
                b.iter(|| black_box(rc_naive(kmers)));
            },
        );

        g.bench_with_input(
            BenchmarkId::new("kme.rs::xor10", 1 << i),
            &xor10_kmers,
            |b, kmers| {
                b.iter(|| black_box(rc_xor10(kmers)));
            },
        );
    }
}

pub fn criterion_benchmark(c: &mut Criterion) {
    construct(c);
    reverse_complement(c);
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
