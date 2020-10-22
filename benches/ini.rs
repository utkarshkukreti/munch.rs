#![cfg_attr(test, feature(test))]
extern crate test;

#[path = "../examples/ini.rs"]
mod ini;

#[bench]
fn bench_ini_example(b: &mut test::Bencher) {
    b.iter(|| ini::parse(test::black_box(ini::EXAMPLE)).unwrap());
    b.bytes = ini::EXAMPLE.len() as u64;
}
