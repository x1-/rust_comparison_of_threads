use std::{thread, time};

use futures::{stream, Future, Stream, Sink};
use futures::future::*;
use futures::sync::oneshot::{self, Sender, Receiver};
use futures::sync::mpsc::{self, unbounded, UnboundedReceiver, UnboundedSender};

use tokio;

pub fn n_tasks(n: usize, sleep_sec: u64) {

    // let mut jhs = Vec::<thread::JoinHandle<_>>::new();
    // (0..n).for_each(|i| {
    //     let jh = thread::spawn(move || {
    //         println!("sleep: {}", i);
    //         thread::sleep(time::Duration::from_secs(sleep_sec));
    //     });
    //     jhs.push(jh);
    // });

    // jhs.into_iter().for_each(|jh| {jh.join().expect("The thread has panicked");});


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

    // tokio::run(lazy(move || {

    //     let mut rxes = Vec::<_>::new();

    //     for i in 0..n {
    //         // let tx = tx.clone();
    //         let (tx, rx) = oneshot::channel();
    //         tokio::spawn(lazy(move || {
    //             thread::sleep(time::Duration::from_secs(sleep_sec));
    //             println!("Send message {}", i);
    //             tx.send(format!("Message {} from spawned task", i)).unwrap();
    //             Ok(())
    //         }));

    //         rxes.push(
    //             rx
    //             // rx.and_then(|msg| {
    //             //     println!("Got `{}`", msg);
    //             //     Ok(())
    //             // })
    //         );
    //     };
    //     let all = join_all(rxes);
    //     let f = all.map(|msgs| {
    //         msgs.iter().for_each(|msg| {
    //             println!("Got `{}`", msg);
    //         })
    //     });
    //     // println!("{:?}", f);
    //     Ok(())
    // }));
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
