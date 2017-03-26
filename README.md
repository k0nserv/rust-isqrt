
Fast Inverse Square Root benchmark
=================================

This repository contains a benchmark between different implementations of the [Fast Inverse Square Root](https://en.wikipedia.org/wiki/Fast_inverse_square_root) in Rust. The methods compared are using `std::mem::compute` and pointer casts. An implementation using `f64::sqrt` is also shown for reference.

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
running 3 tests
test tests::bench_isqrt_no_transmute ... bench: 282,948,645 ns/iter (+/- 49,056,853)
test tests::bench_isqrt_sqrt         ... bench: 965,164,677 ns/iter (+/- 20,426,067)
test tests::bench_isqrt_transmute    ... bench: 273,885,037 ns/iter (+/- 15,464,136)

test result: ok. 0 passed; 0 failed; 0 ignored; 3 measured

```





#### 10 million iterations

````
running 3 tests
test tests::bench_isqrt_no_transmute ... bench:  28,101,823 ns/iter (+/- 3,276,136)
test tests::bench_isqrt_sqrt         ... bench:  96,460,162 ns/iter (+/- 2,735,941)
test tests::bench_isqrt_transmute    ... bench:  27,624,626 ns/iter (+/- 2,690,027)

test result: ok. 0 passed; 0 failed; 0 ignored; 3 measured

````

