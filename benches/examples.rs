use criterion::{black_box, Criterion, Throughput};

#[path = "../examples/arithmetic.rs"]
mod arithmetic;

#[path = "../examples/clojure.rs"]
mod clojure;

#[path = "../examples/ini.rs"]
mod ini;

#[path = "../examples/ip.rs"]
mod ip;

fn bench_arithmetic_example(c: &mut Criterion) {
    let mut group = c.benchmark_group("arithmetic example");

    let str = "3 + -14159 * -2653 / -589 - (79 * 3238 - -462 + (((6))) - -433 - 83279)";
    assert_eq!(
        arithmetic::parse(str).unwrap(),
        3 + -14159 * -2653 / -589 - (79 * 3238 - -462 + (6) - -433 - 83279)
    );

    group.throughput(Throughput::Bytes(str.len() as u64));

    group.bench_function("arithmetic example", |b| {
        b.iter(|| arithmetic::parse(black_box(str)).unwrap());
    });
}

fn bench_clojure_example(c: &mut Criterion) {
    let mut group = c.benchmark_group("clojure example");

    group.throughput(Throughput::Bytes(clojure::EXAMPLE.len() as u64));

    group.bench_function("clojure example", |b| {
        b.iter(|| clojure::parse(black_box(clojure::EXAMPLE)).unwrap());
    });
}

fn bench_ini_example(c: &mut Criterion) {
    let mut group = c.benchmark_group("ini example");

    group.throughput(Throughput::Bytes(ini::EXAMPLE.len() as u64));

    group.bench_function("ini example", |b| {
        b.iter(|| ini::parse(black_box(ini::EXAMPLE)).unwrap());
    });
}

fn bench_ip_example(c: &mut Criterion) {
    static IPS: &[&str] = &["0.0.0.0", "127.0.0.1", "192.168.1.1", "255.255.255.255"];

    let mut group = c.benchmark_group("ip example");

    group.throughput(Throughput::Bytes(
        IPS.iter().map(|ip| ip.len() as u64).sum(),
    ));

    group.bench_function("ip example", |b| {
        b.iter(|| {
            for ip in IPS {
                black_box(ip::parse(black_box(ip)).unwrap());
            }
        });
    });

    group.bench_function("std", |b| {
        b.iter(|| {
            for ip in IPS {
                black_box(black_box(ip).parse::<::std::net::Ipv4Addr>().unwrap());
            }
        });
    });
}

criterion::criterion_group!(
    benches,
    bench_arithmetic_example,
    bench_clojure_example,
    bench_ini_example,
    bench_ip_example,
);

criterion::criterion_main!(benches);
