use criterion::*;
use ecs_bench_suite::*;

fn bench_simple_insert(c: &mut Criterion) {
    let mut group = c.benchmark_group("simple_insert");
    group.bench_function("ellecs", |b| {
        let mut bench = ellecs::simple_insert::Benchmark::new();
        b.iter(move || bench.run());
    });
    group.bench_function("bevy", |b| {
        let mut bench = bevy::simple_insert::Benchmark::new();
        b.iter(move || bench.run());
    });
    group.bench_function("hecs", |b| {
        let mut bench = hecs::simple_insert::Benchmark::new();
        b.iter(move || bench.run());
    });
}

fn bench_simple_iter(c: &mut Criterion) {
    let mut group = c.benchmark_group("simple_iter");
    group.bench_function("ellecs", |b| {
        let mut bench = ellecs::simple_iter::Benchmark::new();
        b.iter(move || bench.run());
    });
    group.bench_function("bevy", |b| {
        let mut bench = bevy::simple_iter::Benchmark::new();
        b.iter(move || bench.run());
    });
    group.bench_function("hecs", |b| {
        let mut bench = hecs::simple_iter::Benchmark::new();
        b.iter(move || bench.run());
    });
}

fn bench_frag_iter_20_bc(c: &mut Criterion) {
    let mut group = c.benchmark_group("fragmented_iter_20");
    group.bench_function("ellecs", |b| {
        let mut bench = ellecs::frag_iter::Benchmark::new();
        b.iter(move || bench.run());
    });
    group.bench_function("bevy", |b| {
        let mut bench = bevy::frag_iter::Benchmark::new();
        b.iter(move || bench.run());
    });
    group.bench_function("hecs", |b| {
        let mut bench = hecs::frag_iter::Benchmark::new();
        b.iter(move || bench.run());
    });
}

fn bench_frag_iter_20_padding_20(c: &mut Criterion) {
    let mut group = c.benchmark_group("frag_iter_20_padding_20");
    group.bench_function("ellecs", |b| {
        let mut bench = ellecs::frag_iter_20_padding_10::Benchmark::new();
        b.iter(move || bench.run());
    });
    group.bench_function("bevy", |b| {
        let mut bench = bevy::frag_iter_20_padding_10::Benchmark::new();
        b.iter(move || bench.run());
    });
    group.bench_function("hecs", |b| {
        let mut bench = hecs::frag_iter_20_padding_10::Benchmark::new();
        b.iter(move || bench.run());
    });
}

fn bench_frag_iter_200_bc(c: &mut Criterion) {
    let mut group = c.benchmark_group("fragmented_iter_200");
    group.bench_function("ellecs", |b| {
        let mut bench = ellecs::frag_iter_200::Benchmark::new();
        b.iter(move || bench.run());
    });
    group.bench_function("bevy", |b| {
        let mut bench = bevy::frag_iter_200::Benchmark::new();
        b.iter(move || bench.run());
    });
    group.bench_function("hecs", |b| {
        let mut bench = hecs::frag_iter_200::Benchmark::new();
        b.iter(move || bench.run());
    });
}

fn bench_frag_iter_2000_bc(c: &mut Criterion) {
    let mut group = c.benchmark_group("fragmented_iter_2000");
    group.bench_function("ellecs", |b| {
        let mut bench = ellecs::frag_iter_2000::Benchmark::new();
        b.iter(move || bench.run());
    });
    group.bench_function("bevy", |b| {
        let mut bench = bevy::frag_iter_2000::Benchmark::new();
        b.iter(move || bench.run());
    });
    group.bench_function("hecs", |b| {
        let mut bench = hecs::frag_iter_2000::Benchmark::new();
        b.iter(move || bench.run());
    });
}

fn bench_add_remove(c: &mut Criterion) {
    let mut group = c.benchmark_group("add_remove_component");
    group.bench_function("ellecs", |b| {
        let mut bench = ellecs::add_remove::Benchmark::new();
        b.iter(move || bench.run());
    });
    group.bench_function("shipyard", |b| {
        let mut bench = shipyard::add_remove::Benchmark::new();
        b.iter(move || bench.run());
    });
    group.bench_function("hecs", |b| {
        let mut bench = hecs::add_remove::Benchmark::new();
        b.iter(move || bench.run());
    });
    group.bench_function("bevy", |b| {
        let mut bench = bevy::add_remove::Benchmark::new();
        b.iter(move || bench.run());
    });
}

fn bench_get(c: &mut Criterion) {
    let mut group = c.benchmark_group("get");
    group.bench_function("hecs", |b| {
        let mut bench = hecs::get::Benchmark::new();
        b.iter(move || bench.run());
    });
    group.bench_function("ellecs", |b| {
        let mut bench = ellecs::get::Benchmark::new();
        b.iter(move || bench.run());
    });
}

criterion_group!(
    benchmarks,
    bench_get,
    bench_simple_insert,
    bench_simple_iter,
    bench_frag_iter_20_bc,
    bench_frag_iter_20_padding_20,
    bench_frag_iter_200_bc,
    bench_frag_iter_2000_bc,
    //bench_schedule,
    //bench_heavy_compute,
    bench_add_remove,
    //bench_serialize_text,
    //bench_serialize_binary,
);
criterion_main!(benchmarks);
