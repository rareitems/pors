use std::cmp::Ordering;

use criterion::{black_box,
                criterion_group,
                criterion_main,
                Criterion};
use makro::X;

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
pub struct Rgb {
    pub r: u8,
    pub g: u8,
    pub b: u8,
}

impl Rgb {
    #[inline]
    pub const fn new(r: u8, g: u8, b: u8) -> Self {
        Self { r, g, b }
    }

    #[inline]
    pub fn dist(self, other: Self) -> f64 {
        let b = f64::from(self.b) - f64::from(other.b);
        let r = f64::from(self.r) - f64::from(other.r);
        let g_pow2 = (f64::from(self.g) - f64::from(other.g)).powi(2);
        r.mul_add(r, b.mul_add(b, g_pow2)).sqrt()
    }

    #[inline]
    pub fn cmp_by_dist(self, other: Self, dist: f64) -> Ordering {
        self.dist(other).partial_cmp(&dist).unwrap()
    }
}

pub struct RgbIter<'a> {
    iter: std::slice::Iter<'a, u8>,
}

impl<'a> Iterator for RgbIter<'a> {
    type Item = Rgb;

    fn next(&mut self) -> Option<Self::Item> {
        if let Some(b) = self.iter.next() {
            let ret = Some(Rgb {
                b: *b,
                g: unsafe { *self.iter.next().unwrap_unchecked() },
                r: unsafe { *self.iter.next().unwrap_unchecked() },
            });
            unsafe { *self.iter.next().unwrap_unchecked() };
            ret
        } else {
            None
        }
    }
}

pub struct RgbIterChunks<'a> {
    iter: std::slice::Chunks<'a, u8>,
}

impl<'a> Iterator for RgbIterChunks<'a> {
    type Item = Rgb;

    fn next(&mut self) -> Option<Self::Item> {
        self.iter.next().map(|chunk| {
            Rgb::new(
                unsafe { *chunk.get_unchecked(2) },
                unsafe { *chunk.get_unchecked(1) },
                unsafe { *chunk.get_unchecked(0) },
            )
        })
    }
}

#[derive(Debug, Default, Clone, PartialEq, Eq)]
pub struct RgbNonCopy {
    pub r: u8,
    pub g: u8,
    pub b: u8,
}

impl RgbNonCopy {
    #[inline]
    pub const fn new(r: u8, g: u8, b: u8) -> Self {
        Self { r, g, b }
    }

    #[inline]
    pub fn dist(&self, other: &Self) -> f64 {
        let b = f64::from(self.b) - f64::from(other.b);
        let r = f64::from(self.r) - f64::from(other.r);
        let g_pow2 = (f64::from(self.g) - f64::from(other.g)).powi(2);
        r.mul_add(r, b.mul_add(b, g_pow2)).sqrt()
    }

    #[inline]
    pub fn cmp_by_dist(&self, other: &Self, dist: f64) -> Ordering {
        self.dist(other).partial_cmp(&dist).unwrap()
    }
}

pub struct NonCopyRgbIter<'a> {
    iter: std::slice::Iter<'a, u8>,
}

impl<'a> Iterator for NonCopyRgbIter<'a> {
    type Item = Rgb;

    fn next(&mut self) -> Option<Self::Item> {
        if let Some(b) = self.iter.next() {
            let ret = Some(Rgb {
                b: *b,
                g: unsafe { *self.iter.next().unwrap_unchecked() },
                r: unsafe { *self.iter.next().unwrap_unchecked() },
            });
            unsafe { *self.iter.next().unwrap_unchecked() };
            ret
        } else {
            None
        }
    }
}

pub fn criterion_benchmark(c: &mut Criterion) {
    let mut group = c.benchmark_group("chunks_vs_other");

    let colors = X::new().get_pixels((0, 0), 300, 1).unwrap();

    let black = Rgb::new(0, 0, 0);

    let mut acc = black_box(0_u64);
    group.bench_function("current", |b| {
        b.iter(|| acc += colors.iter().fold(black_box(0_u64), |acc, it| acc + it.r as u64))
    });

    let a = RgbIter { iter: colors.0.iter() };

    a.filter(|p| p.cmp_by_dist(black, 5.0).is_lt()).count();

    let b = NonCopyRgbIter { iter: colors.0.iter() };

    b.filter(|p| p.cmp_by_dist(black, 5.0).is_lt()).count();

    let mut acc = black_box(0_u64);
    group.bench_function("current", |b| {
        b.iter(|| acc += colors.iter().fold(black_box(0_u64), |acc, it| acc + it.r as u64))
    });

    group.finish()
}

criterion_group!(
    name = benches;
    config = Criterion::default();
    targets = criterion_benchmark
);
criterion_main!(benches);
