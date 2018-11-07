/**
 * futures-cpupool
 * https://crates.io/crates/futures-cpupool
 * https://docs.rs/futures-cpupool/0.1.8/futures_cpupool/
 */
use std::{thread, time};

use futures::future::*;
use futures::Future;
use futures_cpupool::{CpuFuture, CpuPool};

pub fn n_tasks(n: usize, sleep_sec: u64) {
    // Create a worker thread pool with four threads
    let pool = CpuPool::new(n);

    let tasks = (0..n).map(|i| {
        pool.spawn_fn(move ||{
            thread::sleep(time::Duration::from_secs(sleep_sec));
            ok::<usize, String>(i)
        })
    });

    // Express some further computation once the work is completed on the thread
    // pool.
    // let c = a.join(b).map(|(a, b)| a + b).wait().unwrap();
    // let c = task.wait().unwrap();
    let f = join_all(tasks);
    let f = f.map(|xs| {
        xs.iter().sum::<usize>()
    }).wait();
    // Print out the result
    println!("{:?}", f);
}