#![cfg_attr(test, feature(test))]
extern crate test;

#[path = "../examples/clojure.rs"]
mod clojure;

#[bench]
fn bench_clojure_example(b: &mut test::Bencher) {
    b.iter(|| clojure::parse(test::black_box(clojure::EXAMPLE)).unwrap());
    b.bytes = clojure::EXAMPLE.len() as u64;
}
