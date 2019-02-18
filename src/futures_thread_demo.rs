use std::cell::{Cell, RefCell};
use std::pin::Pin;
use std::rc::Rc;
use std::{thread, time};

use futures::prelude::*;
use futures::executor::ThreadPoolBuilder;
use futures::future::{Future, lazy};
use futures::task::{LocalWaker, Poll, SpawnExt};

struct SlowComputation {
    sleep_sec: u64
}

impl Future for SlowComputation {
    type Output = ();

    fn poll(self: Pin<&mut Self>, lw: &LocalWaker) -> Poll<Self::Output> {
        let secs = time::Duration::from_secs(self.sleep_sec);
        thread::sleep(secs);

        Poll::Ready(())
    }
}


pub fn n_tasks(n: usize, sleep_sec: u64) {

    let cnt = Rc::new(Cell::new(0));

    let mut pool = ThreadPoolBuilder::new()
        .pool_size(n)
        .create()
        .expect("Failed to create threadpool");

    for i in 0..n {
        let fut = SlowComputation { sleep_sec: sleep_sec };
        if i == (n - 1) {
            pool.run(fut);
        } else {
            pool.spawn(fut);
        }
    }

    println!("{:?}", cnt.get());
}

