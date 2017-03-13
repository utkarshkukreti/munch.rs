#![cfg_attr(test, feature(test))]
extern crate munch;
extern crate test;

#[path = "../examples/ip.rs"]
mod ip;

static IPS: &[&'static str] = &["0.0.0.0", "127.0.0.1", "192.168.1.1", "255.255.255.255"];

#[bench]
fn bench_ip_example(b: &mut test::Bencher) {
    b.iter(|| for ip in IPS {
        test::black_box(ip::parse(test::black_box(ip)).unwrap());
    });
    b.bytes = IPS.iter().map(|ip: &&str| ip.len()).sum::<usize>() as u64;
}

#[bench]
fn bench_ip_std(b: &mut test::Bencher) {
    b.iter(|| for ip in IPS {
        test::black_box(test::black_box(ip).parse::<::std::net::Ipv4Addr>().unwrap());
    });
    b.bytes = IPS.iter().map(|ip: &&str| ip.len()).sum::<usize>() as u64;
}
