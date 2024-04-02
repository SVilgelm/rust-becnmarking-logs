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
test tests::bench_format        ... bench:         527 ns/iter (+/- 29)
test tests::bench_kv            ... bench:         573 ns/iter (+/- 36)
test tests::bench_kv_10         ... bench:       1,136 ns/iter (+/- 134)
test tests::bench_kv_with_error ... bench:         648 ns/iter (+/- 45)
test tests::bench_stat          ... bench:         446 ns/iter (+/- 51)

test result: ok. 0 passed; 0 failed; 0 ignored; 5 measured; 0 filtered out; finished in 6.24s

     Running unittests src\lib.rs (target\release\deps\print_logger-8964df2620bf5d6a.exe)

running 5 tests
test tests::bench_format        ... bench:          76 ns/iter (+/- 14)
test tests::bench_kv            ... bench:         211 ns/iter (+/- 8)
test tests::bench_kv_10         ... bench:       1,981 ns/iter (+/- 52)
test tests::bench_kv_with_error ... bench:         526 ns/iter (+/- 138)
test tests::bench_stat          ... bench:          20 ns/iter (+/- 1)

test result: ok. 0 passed; 0 failed; 0 ignored; 5 measured; 0 filtered out; finished in 4.36s

     Running unittests src\lib.rs (target\release\deps\structured_logger-c3d78b8b6e1514d3.exe)

running 5 tests
test tests::bench_format        ... bench:         456 ns/iter (+/- 67)
test tests::bench_kv            ... bench:         431 ns/iter (+/- 10)
test tests::bench_kv_10         ... bench:       1,170 ns/iter (+/- 253)
test tests::bench_kv_with_error ... bench:         528 ns/iter (+/- 323)
test tests::bench_stat          ... bench:         329 ns/iter (+/- 18)
```
