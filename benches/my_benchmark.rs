#![allow(unused)]

use criterion::{black_box, criterion_group, criterion_main, Criterion};
use template_crate_with_criterion_benchmark::adding_i32;


pub fn criterion_benchmark(c: &mut Criterion){
    c.bench_function("adding i32 values", |b| b.iter(|| adding_i32(10,10)));
    c.bench_function("power of two", |b| b.iter(||
        custom_benches::benchmarking_pow2(black_box(10))));
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);



mod custom_benches {
    pub(crate) fn benchmarking_pow2(k:u32){
        u32::pow(k, 2);
    }
}
