use criterion::{black_box,
                criterion_group,
                criterion_main,
                Criterion};
use makro::X;
// use poe::inventory::INV_POS;

static POE_WM_NAME: [u8; 13] = *b"Path of Exile";
const POE_WM_NAME_CONST: [u8; 13] = *b"Path of Exile";

pub fn criterion_benchmark(c: &mut Criterion) {
    let mut group = c.benchmark_group("wm_name");

    let x = X::new();

    let mut acc = black_box(0_u64);
    group.bench_function("static", |b| {
        b.iter(|| {
            if x.get_window_name_specific_size(POE_WM_NAME.len()).unwrap().value.eq(&POE_WM_NAME) {
                acc += 1;
            }
        })
    });

    let mut acc = black_box(0_u64);
    group.bench_function("const", |b| {
        b.iter(|| {
            if x.get_window_name_specific_size(POE_WM_NAME_CONST.len())
                .unwrap()
                .value
                .eq(&POE_WM_NAME_CONST)
            {
                acc += 1;
            }
        })
    });
}

criterion_group!(
    name = benches;
    config = Criterion::default();
    targets = criterion_benchmark
);
criterion_main!(benches);
