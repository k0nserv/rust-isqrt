
Fast Inverse Square Root benchmark
=================================

This repository contains a benchmark between different implementations of the [Fast Inverse Square Root](https://en.wikipedia.org/wiki/Fast_inverse_square_root) in Rust. The methods compared are using `std::mem::compute` and pointer casts.

This uses [benchmark tests](https://doc.rust-lang.org/book/benchmark-tests.html) which are at the time of writing not stable so to run them you need a nightly build of rust. Installing and using nightly builds is covered in the [rustup docs](https://github.com/rust-lang-nursery/rustup.rs#working-with-nightly-rust);


Running
-------

With a nightly rust activated run the following

```bash
cargo bench
```

By default this will calculate 100 million inverse square roots per iteration. This value can be overridden with the environment variable `NUM_ITERATIONS`.

```bash
NUM_ITERATIONS=100 cargo bench # Using 100 calculations per iteration
```


Results
-------

### Macbook pro 13" mid 2012 2.5Ghz i5 2 cores



#### 100 million iterations

```
running 2 tests
test tests::bench_isqrt_no_transmute ... bench: 274,405,073 ns/iter (+/- 11,466,661)
test tests::bench_isqrt_transmute    ... bench: 272,768,130 ns/iter (+/- 7,655,082)

test result: ok. 0 passed; 0 failed; 0 ignored; 2 measured
```





#### 10 million iterations

````
running 2 tests
test tests::bench_isqrt_no_transmute ... bench:  27,098,173 ns/iter (+/- 1,584,364)
test tests::bench_isqrt_transmute    ... bench:  27,409,085 ns/iter (+/- 2,957,428)

test result: ok. 0 passed; 0 failed; 0 ignored; 2 measured

````

