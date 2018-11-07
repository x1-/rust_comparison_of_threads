/**
 * rayon
 * https://github.com/rayon-rs/rayon
 * https://docs.rs/rayon/1.0.3/rayon/
 */
use std::{thread, time};

use rayon;
use rayon::prelude::*;

pub fn n_tasks(n: usize, sleep_sec: u64) {
    // Create a worker thread pool with four threads
    let pool = rayon::ThreadPoolBuilder::new().num_threads(n).build().unwrap();

    let task = (0..n)
        .into_par_iter()
        .map(|i| {
            thread::sleep(time::Duration::from_secs(sleep_sec));
            i
        })
        .sum::<usize>();
    println!("{:?}", task);
}
