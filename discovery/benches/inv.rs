use criterion::{black_box,
                criterion_group,
                criterion_main,
                Criterion};
use makro::{Pos,
            Rgb,
            X};

pub static INV_POS: [Pos; 60] = [
    Pos { x: 1322, y: 589 },
    Pos { x: 1375, y: 589 },
    Pos { x: 1428, y: 589 },
    Pos { x: 1480, y: 589 },
    Pos { x: 1533, y: 589 },
    Pos { x: 1586, y: 589 },
    Pos { x: 1638, y: 589 },
    Pos { x: 1691, y: 589 },
    Pos { x: 1744, y: 589 },
    Pos { x: 1796, y: 589 },
    Pos { x: 1849, y: 589 },
    Pos { x: 1902, y: 589 },
    Pos { x: 1322, y: 642 },
    Pos { x: 1375, y: 642 },
    Pos { x: 1428, y: 642 },
    Pos { x: 1480, y: 642 },
    Pos { x: 1533, y: 642 },
    Pos { x: 1586, y: 642 },
    Pos { x: 1638, y: 642 },
    Pos { x: 1691, y: 642 },
    Pos { x: 1744, y: 642 },
    Pos { x: 1796, y: 642 },
    Pos { x: 1849, y: 642 },
    Pos { x: 1902, y: 642 },
    Pos { x: 1322, y: 694 },
    Pos { x: 1375, y: 694 },
    Pos { x: 1428, y: 694 },
    Pos { x: 1480, y: 694 },
    Pos { x: 1533, y: 694 },
    Pos { x: 1586, y: 694 },
    Pos { x: 1638, y: 694 },
    Pos { x: 1691, y: 694 },
    Pos { x: 1744, y: 694 },
    Pos { x: 1796, y: 694 },
    Pos { x: 1849, y: 694 },
    Pos { x: 1902, y: 694 },
    Pos { x: 1322, y: 747 },
    Pos { x: 1375, y: 747 },
    Pos { x: 1428, y: 747 },
    Pos { x: 1480, y: 747 },
    Pos { x: 1533, y: 747 },
    Pos { x: 1586, y: 747 },
    Pos { x: 1638, y: 747 },
    Pos { x: 1691, y: 747 },
    Pos { x: 1744, y: 747 },
    Pos { x: 1796, y: 747 },
    Pos { x: 1849, y: 747 },
    Pos { x: 1902, y: 747 },
    Pos { x: 1322, y: 800 },
    Pos { x: 1375, y: 800 },
    Pos { x: 1428, y: 800 },
    Pos { x: 1480, y: 800 },
    Pos { x: 1533, y: 800 },
    Pos { x: 1586, y: 800 },
    Pos { x: 1638, y: 800 },
    Pos { x: 1691, y: 800 },
    Pos { x: 1744, y: 800 },
    Pos { x: 1796, y: 800 },
    Pos { x: 1849, y: 800 },
    Pos { x: 1902, y: 800 },
];

/// Color of red inventory square
pub const UNID: Rgb = Rgb::new(44, 5, 3);

/// Color of empty inventory square
pub const NONE: Rgb = Rgb::new(13, 10, 5);

/// Color of inventory square with something in it
pub const SOME: Rgb = Rgb::new(7, 5, 29);

pub fn criterion_benchmark(c: &mut Criterion) {
    let mut group = c.benchmark_group("chunks_vs_other");

    let x = X::new();

    group.bench_function("iter", |b| {
        b.iter(|| {
            let mut pos_some = black_box(Vec::with_capacity(INV_POS.len()));
            for pos in INV_POS {
                let color = x.get_pixel(pos).unwrap();
                if color.cmp_by_dist(SOME, 5.0).is_le() {
                    pos_some.push(pos);
                }
                if color.cmp_by_dist(UNID, 5.0).is_le() {
                    pos_some.push(pos);
                }
            }
        })
    });

    group.bench_function("func", |b| {
        b.iter(|| {
            let _pos_some: Vec<_> = black_box(
                INV_POS
                    .iter()
                    .filter(|pos| {
                        let color = x.get_pixel(**pos).unwrap();

                        if color.cmp_by_dist(SOME, 5.0).is_le() {
                            return true;
                        }

                        if color.cmp_by_dist(UNID, 5.0).is_le() {
                            return true;
                        }

                        false
                    })
                    .collect(),
            );
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
