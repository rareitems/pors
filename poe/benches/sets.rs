use std::collections::{BTreeSet,
                       HashSet};

use criterion::{black_box,
                criterion_group,
                criterion_main,
                Criterion};
use makro::Pos;
use poe::inventory::INV_POS;

pub fn criterion_benchmark(c: &mut Criterion) {
    let mut group = c.benchmark_group("sets");

    // let colors = X::new().get_pixels((0, 0), 300, 1).unwrap();

    let sqs = INV_POS;

    let mut hash = HashSet::new();
    // hash.insert(sqs[0]);
    hash.insert(sqs[55]);
    hash.insert(sqs[10]);
    // hash.insert(sqs[15]);
    // hash.insert(sqs[50]);
    //
    // hash.insert(sqs[1]);
    // hash.insert(sqs[2]);
    // hash.insert(sqs[3]);
    // hash.insert(sqs[4]);
    // hash.insert(sqs[5]);
    // hash.insert(sqs[6]);

    let mut tree = BTreeSet::new();
    // tree.insert(sqs[0]);
    tree.insert(sqs[55]);
    tree.insert(sqs[10]);
    // tree.insert(sqs[15]);
    // tree.insert(sqs[50]);
    //
    // tree.insert(sqs[1]);
    // tree.insert(sqs[2]);
    // tree.insert(sqs[3]);
    // tree.insert(sqs[4]);
    // tree.insert(sqs[5]);
    // tree.insert(sqs[6]);

    let mut arr: arrayvec::ArrayVec<Pos, 2> = arrayvec::ArrayVec::new();
    // arr.push(sqs[0]);
    arr.push(sqs[55]);
    arr.push(sqs[10]);
    // arr.push(sqs[15]);
    // arr.push(sqs[50]);
    //
    // arr.push(sqs[1]);
    // arr.push(sqs[2]);
    // arr.push(sqs[3]);
    // arr.push(sqs[4]);
    // arr.push(sqs[5]);
    // arr.push(sqs[6]);

    let mut acc = black_box(0_u64);
    group.bench_function("hash", |b| {
        b.iter(|| {
            for pos in sqs {
                if hash.get(&pos).is_some() {
                    acc += 1;
                }
            }
        })
    });

    let mut acc = black_box(0_u64);
    group.bench_function("tree", |b| {
        b.iter(|| {
            for pos in sqs {
                if tree.get(&pos).is_some() {
                    acc += 1;
                }
            }
        })
    });

    let mut acc = black_box(0_u64);
    group.bench_function("arrayvec", |b| {
        b.iter(|| {
            for pos in sqs {
                if arr.contains(&pos) {
                    acc += 1;
                }
            }
        })
    });

    group.finish()
}

criterion_group!(
    name = benches;
    config = Criterion::default();
    targets = criterion_benchmark
);
criterion_main!(benches);
