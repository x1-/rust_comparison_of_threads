/**
 * futures-preview
 * https://github.com/rust-lang-nursery/futures-rs
 * https://crates.rs/crates/futures-preview
 */
use std::{thread, time};

use futures::executor::ThreadPoolBuilder;
use core::pin::Pin;
use futures::prelude::*;
use futures::task::{Poll, LocalWaker, SpawnExt};

struct SlowComputation {
    index    : usize,
    sleep_sec: u64
}
impl SlowComputation {
    fn new(n: usize, sleep_sec: u64) -> Self {
        Self {
            index    : n,
            sleep_sec: sleep_sec
        }
    }
}
impl Future for SlowComputation {
    type Output = ();

    fn poll(self: Pin<&mut Self>, lw: &LocalWaker) -> Poll<Self::Output> {
        thread::sleep(time::Duration::from_secs(self.sleep_sec));
        Poll::Ready(())
    }
}

pub fn n_tasks(n: usize, sleep_sec: u64) {

    let mut pool = ThreadPoolBuilder::new()
        .pool_size(n)
        .create()
        .expect("Failed to create threadpool");

    (0..(n-1)).for_each(|i| {
        pool.spawn(SlowComputation::new(i, sleep_sec));
    });

    let f = pool.run(SlowComputation::new(n, sleep_sec));
    println!("{:?}", f);
}
