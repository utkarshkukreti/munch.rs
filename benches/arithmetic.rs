#![cfg_attr(test, feature(test))]
extern crate munch;
extern crate test;

#[path = "../examples/arithmetic.rs"]
mod arithmetic;

#[bench]
fn bench_arithmetic_example(b: &mut test::Bencher) {
    let str = "3 + -14159 * -2653 / -589 - (79 * 3238 - -462 + (((6))) - -433 - 83279)";
    assert_eq!(arithmetic::parse(str).unwrap(),
               3 + -14159 * -2653 / -589 - (79 * 3238 - -462 + (((6))) - -433 - 83279));
    b.iter(|| arithmetic::parse(test::black_box(str)).unwrap());
    b.bytes = str.len() as u64;
}
