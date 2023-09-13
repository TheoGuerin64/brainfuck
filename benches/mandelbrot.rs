use brainfuck_interpreter::interpret;
use criterion::{black_box, criterion_group, criterion_main, Criterion};
use std::time::Duration;

fn criterion_benchmark(c: &mut Criterion) {
    let mut group = c.benchmark_group("fast");
    group.significance_level(0.2).sample_size(20);
    group.measurement_time(Duration::from_secs(160));
    group.bench_function("mandelbrot", |b| {
        b.iter(|| interpret(black_box("./assets/mandelbrot.bf")))
    });
    group.finish();
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
