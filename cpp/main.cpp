#include <vector>
#include <iostream>
#include <stdint.h>
#include <random>
#include <benchmark/benchmark.h>


inline uint32_t ip_bin(uint64_t *x, uint64_t *y) {
    uint64_t ret = 0;
    for (int i = 0; i < 16; i++) {
        ret += __builtin_popcountll(x[i] & y[i]);
    }
    return ret;
}

uint32_t ip_byte_bin(uint64_t *x, uint64_t *y) {
    uint64_t ret = 0;
    for (int i = 0; i < 4; i++) {
        ret += ip_bin(x, y) << i;
        y += 16;
    }
    return ret;   
}

static void BM_IP_BIN(benchmark::State& state) {
    uint64_t x[4];
    uint64_t y[16];
    uint64_t z[64];
    std::mt19937_64 rng;
    for (int i = 0; i < 4; i++) {
        x[i] = rng();
    }
    for (int i = 0; i < 16; i++) {
        y[i] = rng();
    }
    for (int i = 0; i < 64; i++) {
        z[i] = rng();
    }

    for (auto _ : state) {
        // benchmark::DoNotOptimize(ip_byte_bin(x, y));
        benchmark::DoNotOptimize(ip_byte_bin(y, z));
    }
}

BENCHMARK(BM_IP_BIN);
BENCHMARK_MAIN();

// g++ -march=core-avx2 -Ofast main.cpp -isystem benchmark/include -lpthread -L ../../google_benchmark/build/src -lbenchmark -o bench
// 256: 3.46 ns
// 1024: 12.8 ns