/**
 * tokio-threadpool
 * https://github.com/tokio-rs/tokio/tree/master/tokio-threadpool
 * https://docs.rs/tokio-threadpool/0.1.8/tokio_threadpool/
 */
use std::{thread, time};

use futures::future::*;
use futures::sync::oneshot;
use futures::{Future, lazy};

use tokio_threadpool::{Builder, ThreadPool};


pub fn n_tasks(n: usize, sleep_sec: u64) {
    // Create a worker thread pool with four threads
    let pool = Builder::new()
        .pool_size(n)
        // .keep_alive(Some(Duration::from_secs(30)))
        .build();


    let tasks = (0..n).map(|i| {
        pool.spawn_handle(lazy(move || {
            thread::sleep(time::Duration::from_secs(sleep_sec));
            ok::<usize, String>(i)
        }))
    });

    // Express some further computation once the work is completed on the thread
    let f = join_all(tasks);
    let f = f.map(|xs| {
        xs.iter().sum::<usize>()
    }).wait();
    // Print out the result
    println!("{:?}", f);
}
