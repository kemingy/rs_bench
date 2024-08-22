use criterion::criterion_main;

mod benchmarks;

criterion_main! {
    benchmarks::binary_ip::binary_ip,
    benchmarks::quantize::quantize,
    benchmarks::binarize::binarize,
}
