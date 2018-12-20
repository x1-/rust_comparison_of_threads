use std::{thread, time};

use futures::{stream, Future, Stream, Sink};
use futures::future::*;
use futures::sync::oneshot;
use futures::sync::mpsc::unbounded;

use tokio;

pub fn n_tasks(n: usize, sleep_sec: u64) {

    tokio::run(lazy(move || {
        for i in 0..n {
            tokio::spawn(lazy(move || {
                thread::sleep(time::Duration::from_secs(sleep_sec));
                println!("tokio task {}", i);
                Ok(())
            }));
        }
        Ok(())
    }));
    println!("end!");
}

pub fn tokio_oneshot(n: usize, sleep_sec: u64) {

    tokio::run(lazy(move || {
        let (tx, rx) = oneshot::channel();

        tokio::spawn(lazy(move || {
            println!("Send message after sleeping {} sec.", sleep_sec);
            thread::sleep(time::Duration::from_secs(sleep_sec));
            tx.send("hello from spawned task");
            Ok(())
        }));

        rx.and_then(|msg| {
            println!("Got `{}`", msg);
            Ok(())
        })
        .map_err(|e| println!("error = {:?}", e))
    }));
    println!("end!");
}

pub fn tokio_mpsc(n: usize, sleep_sec: u64) {

    tokio::run(lazy(move || {
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
