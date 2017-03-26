#![feature(test)]
extern crate test;

use std::mem::transmute;

fn isqrt_transmute(value: f64) -> f64 {
    let mut y = value;
    let x2 = y * 0.5;
    let mut i: u64 = unsafe { transmute(y) };
    i = 0x5fe6eb50c7b537a9 - (i >> 1);

    y = unsafe { transmute(i) };
    y = y * (1.5 - (x2 * y * y));

    y
}

fn isqrt_no_transmute(value: f64) -> f64 {
    let mut y = value;
    let x2 = y * 0.5;
    let i: *mut u64 = unsafe { &mut *(&mut y as *mut f64 as *mut u64) };
    unsafe { *i = 0x5fe6eb50c7b537a9 - (*i >> 1) };

    y = unsafe { *(i as *mut f64) };
    y = y * (1.5 - (x2 * y * y));

    y
}


#[cfg(test)]
mod tests {
    use super::*;
    use test::Bencher;
    use std::env;
    const DEFAULT_NUM_ITERATIONS: u64 = 1_000_000_00;

    fn num_iterations() -> u64 {
        match env::var("NUM_ITERATIONS") {
            Ok(iterations) => iterations.parse::<u64>().unwrap_or(DEFAULT_NUM_ITERATIONS),
            Err(_) => DEFAULT_NUM_ITERATIONS,
        }
    }

    #[bench]
    fn bench_isqrt_transmute(b: &mut Bencher) {
        b.iter(|| {
                   let n = test::black_box(num_iterations());

                   (0..n).fold(0.0, |accum, i| (accum as f64) + isqrt_transmute(i as f64))
               });
    }

    #[bench]
    fn bench_isqrt_no_transmute(b: &mut Bencher) {
        b.iter(|| {
                   let n = test::black_box(num_iterations());

                   (0..n).fold(0.0,
                               |accum, i| (accum as f64) + isqrt_no_transmute(i as f64))
               });
    }
}
