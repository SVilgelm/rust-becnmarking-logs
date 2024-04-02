# Benchmarking different logging crates

## Supported crates

- `print_logger` is not real crate, just a simple implementation to print out the logs
- [env_logger](https://lib.rs/crates/env_logger)
- [structured-logger](https://lib.rs/crates/structured-logger)

## Benchmarks

- The `stat` is used to log a static text.
- The `format` is used to log using formatting
- The `kv` is used to log using key = value pairs
- The `kv_with_error` is used to log using key = value pairs, when one pair is an error
- The `kv_10` is used to log using ten key = value pairs

Command to run the benchmarks:

```shell
cargo +nightly bench
```

Result:

```rust
     Running unittests src\lib.rs (target\release\deps\env_logger-aeb97c73992fea56.exe)

running 5 tests
test tests::bench_format        ... bench:         542 ns/iter (+/- 229)
test tests::bench_kv            ... bench:         579 ns/iter (+/- 44)
test tests::bench_kv_10         ... bench:       1,170 ns/iter (+/- 690)
test tests::bench_kv_with_error ... bench:         651 ns/iter (+/- 27)
test tests::bench_stat          ... bench:         439 ns/iter (+/- 29)

test result: ok. 0 passed; 0 failed; 0 ignored; 5 measured; 0 filtered out; finished in 7.28s

     Running unittests src\lib.rs (target\release\deps\print_logger-8964df2620bf5d6a.exe)

running 5 tests
test tests::bench_format        ... bench:         210 ns/iter (+/- 87)
test tests::bench_kv            ... bench:         340 ns/iter (+/- 69)
test tests::bench_kv_10         ... bench:         732 ns/iter (+/- 173)
test tests::bench_kv_with_error ... bench:         446 ns/iter (+/- 41)
test tests::bench_stat          ... bench:         152 ns/iter (+/- 13)

test result: ok. 0 passed; 0 failed; 0 ignored; 5 measured; 0 filtered out; finished in 13.61s

     Running unittests src\lib.rs (target\release\deps\structured_logger-c3d78b8b6e1514d3.exe)

running 5 tests
test tests::bench_format        ... bench:         472 ns/iter (+/- 181)
test tests::bench_kv            ... bench:         812 ns/iter (+/- 656)
test tests::bench_kv_10         ... bench:       1,263 ns/iter (+/- 764)
test tests::bench_kv_with_error ... bench:         527 ns/iter (+/- 128)
test tests::bench_stat          ... bench:         339 ns/iter (+/- 61)
```
