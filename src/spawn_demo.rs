use std::{thread, time};

use futures::{stream, Future, Stream, Sink};
use futures::future::lazy;
use futures::sync::mpsc;
use futures::sync::mpsc::{unbounded, UnboundedReceiver, UnboundedSender};

use tokio;

pub fn n_tasks(n: usize, sleep_sec: u64) {

    let mut jhs = Vec::<thread::JoinHandle<_>>::new();
    (0..n).for_each(|i| {
        let jh = thread::spawn(move || {
            println!("sleep: {}", i);
            thread::sleep(time::Duration::from_secs(sleep_sec));
        });
        jhs.push(jh);
    });

    jhs.into_iter().for_each(|jh| {jh.join().expect("The thread has panicked");});

    println!("end!");
}

pub fn tokio_spawn(n: usize, sleep_sec: u64) {

    tokio::run(lazy(move || {
        // let (tx, rx) = mpsc::channel(1_024);
        let (tx, rx) = unbounded();

        (0..n).for_each(|i| {
            let tx = tx.clone();
            tokio::spawn({
                stream::iter_ok::<_, ()>(vec![0]).fold(tx, move |tx, n| {
                    thread::sleep(time::Duration::from_secs(sleep_sec));
                    tx.send(format!("Message {}-{} from spawned task", i, n))
                        .map_err(|e| println!("error = {:?}", e))
                })
                .map(|_| ()) // Drop tx handle
            });
        });

        rx.for_each(|msg| {
            println!("Got `{}`", msg);
            Ok(())
        })
    }));
    println!("end!");
}
