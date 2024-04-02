# Benchmarking different logging crates

## Supported crates

- `print_logger` is not real crate, just a simple implementation to print out the logs
- [env_logger](https://lib.rs/crates/env_logger)
- [structured-logger](https://lib.rs/crates/structured-logger)

## Benchmarks

Command to run the benchmarks:

```shell
cargo +nightly bench
```

Result:

```rust
     Running unittests src\lib.rs (target\release\deps\env_logger-febe89d92171346b.exe)

running 3 tests
test tests::bench_format ... bench:         522 ns/iter (+/- 64)
test tests::bench_kv     ... bench:         571 ns/iter (+/- 40)
test tests::bench_stat   ... bench:         434 ns/iter (+/- 206)

test result: ok. 0 passed; 0 failed; 0 ignored; 3 measured; 0 filtered out; finished in 3.94s

     Running unittests src\lib.rs (target\release\deps\print_logger-8964df2620bf5d6a.exe)

running 3 tests
test tests::bench_format ... bench:          75 ns/iter (+/- 6)
test tests::bench_kv     ... bench:         219 ns/iter (+/- 107)
test tests::bench_stat   ... bench:          20 ns/iter (+/- 3)

test result: ok. 0 passed; 0 failed; 0 ignored; 3 measured; 0 filtered out; finished in 6.12s

     Running unittests src\lib.rs (target\release\deps\structured_logger-6055a01d33abc706.exe)

running 3 tests
test tests::bench_format ... bench:         452 ns/iter (+/- 11)
test tests::bench_kv     ... bench:         430 ns/iter (+/- 34)
test tests::bench_stat   ... bench:         327 ns/iter (+/- 105)
```
