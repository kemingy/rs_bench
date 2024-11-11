use criterion::criterion_main;

mod benchmarks;

#[cfg(not(feature = "linear-map"))]
criterion_main! {
    benchmarks::binary_ip::binary_ip,
    benchmarks::quantize::quantize,
    benchmarks::binarize::binarize,
    benchmarks::minmax::minmax,
    benchmarks::f32_cmp::f32_cmp,
    benchmarks::projection::project,
    benchmarks::norm::norm,
}

#[cfg(feature = "linear-map")]
criterion_main! {
    benchmarks::binary_ip::binary_ip,
    benchmarks::quantize::quantize,
    benchmarks::binarize::binarize,
    benchmarks::minmax::minmax,
    benchmarks::f32_cmp::f32_cmp,
    benchmarks::projection::project,
    benchmarks::norm::norm,
    benchmarks::linear::linear,
}
