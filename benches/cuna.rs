use criterion::{black_box, criterion_group, criterion_main, Criterion};
use cuna::CueSheet;
use std::io::Cursor;
use std::str::FromStr;

const CUE: &str = include_str!(r"LACM-34919.cue");
const PATH: &str = r"benches/LACM-34919.cue";

pub fn cuna_bench(c: &mut Criterion) {
    c.bench_function("cuna", |b| {
        b.iter(|| CueSheet::from_str(black_box(CUE)).unwrap())
    });
}
pub fn cuna_buf_read_bench(c: &mut Criterion) {
    let mut buf = Cursor::new(black_box(CUE));
    c.bench_function("cuna_buf_read", |b| {
        b.iter(|| CueSheet::from_buf_read(black_box(&mut buf)).unwrap())
    });
}
pub fn cuna_open_bench(c: &mut Criterion) {
    c.bench_function("cuna_open", |b| {
        b.iter(|| CueSheet::open(black_box(PATH)).unwrap())
    });
}

criterion_group!(benches, cuna_bench, cuna_buf_read_bench, cuna_open_bench);
criterion_main!(benches);
