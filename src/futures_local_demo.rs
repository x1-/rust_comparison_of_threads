use std::cell::{Cell, RefCell};
use std::pin::Pin;
use std::rc::Rc;
use std::{thread, time};

use futures::executor::LocalPool;
use futures::future::*;
use futures::task::{LocalWaker, Poll, Spawn, LocalSpawn};



pub fn n_tasks(n: usize, sleep_sec: u64) {

    let cnt = Rc::new(Cell::new(0));

    let mut pool = LocalPool::new();
    let mut spawn = pool.spawner();

    for _ in 0..n {
        let cnt = cnt.clone();
        spawn.spawn_local_obj(Box::pin(lazy(move |_| {
            cnt.set(cnt.get() + 1);
            thread::sleep(time::Duration::from_secs(sleep_sec));
            ()
        })).into()).unwrap();
    }

    pool.run();


    println!("{:?}", cnt.get());
}

