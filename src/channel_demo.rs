use std::{thread, time};

// use futures::future::*;
// use futures::sync::mpsc;

// use futures::sync::mpsc::{unbounded, UnboundedReceiver, UnboundedSender};
// use futures::sync::oneshot;
// use futures::{future, Async, Future, Poll, Stream};

use std::sync::mpsc::{self, Sender, Receiver};

pub fn n_tasks(n: usize, sleep_sec: u64) {
    /** V1 */
    // let mut txes = Vec::<(UnboundedSender<usize>, usize)>::new();
    // let mut rxes = Vec::<UnboundedReceiver<usize>>::new();
    // (0..n).for_each(|i| {
    //     println!("tasks: {}", i);
    //     let (tx, rx) = unbounded();
    //     txes.push((tx, i));
    //     rxes.push(rx);
    // });

    // // let txes = tasks.clone().map(move |(t, _)| t);
    // // let mut rxes = tasks.map(move |(_, r)| r).collect::<Vec<UnboundedReceiver<usize>>>();

    // // let ref ss = txes.into_iter();
    // let senders = txes.into_iter().map(|(tx, i)| {
    //     thread::spawn(move || {
    //         println!("sleep: {}", i);
    //         thread::sleep(time::Duration::from_secs(sleep_sec));
    //         tx.unbounded_send(i.clone()).expect("Unable to send on channel");
    //         // tx.send(()).unwrap();
    //     })
    // });

    // // loop {
    // //     rxes.retain(|ref mut rx| {
    // //         match rx.poll() {
    // //             Ok(Async::Ready(Some(n))) => false,
    // //             Ok(Async::NotReady) => true,
    // //             Ok(Async::Ready(None)) | Err(_) => false,
    // //         }
    // //     });
    // //     if rxes.len() == 0 { break; }
    // // };

    // // while let Some(mut rx) = rxes.iter_mut().next() {
    // //     match rx.poll() {
    // //         Ok(Async::Ready(Some(n))) => {},
    // //         Ok(Async::NotReady) => rxes.push(*rx),
    // //         Ok(Async::Ready(None)) | Err(_) => {},
    // //     };
    // // };

    // println!("52");

    // let counter = 0;
    // let mut notreadise = rxes;
    // loop {
    //     println!("loop start");
    //     let mut unrecvs = notreadise.iter_mut();
    //     let mut notreadise = Vec::new();
    //     while let Some(rx) = unrecvs.next() {
    //         println!("while let: {:?}", rx);
    //         let counter = counter + 1;
    //         match rx.poll() {
    //             Ok(Async::NotReady) => {
    //                 println!("Async::NotReady: {}", counter);
    //                 notreadise.push(rx)
    //             },
    //             _ => {
    //                 println!("loop.other: {}", counter);
    //             },
    //         }
    //     };
    //     if notreadise.len() == 0 { break; }
    //     //     match maybe_rx {
    //     //         Some(mut rx) => {
    //     //             match rx.poll() {
    //     //                 Ok(Async::Ready(Some(n))) => None,
    //     //                 Ok(Async::NotReady) => Some(rx),
    //     //                 Ok(Async::Ready(None)) | Err(_) => None,
    //     //             }
    //     //         },
    //     //         None => None
    //     //     }
    //     // });
    //     // if rxes.count() == 0 { break; }
    // };

    // senders.for_each(|jh| {jh.join().expect("The sender thread has panicked");});


    /** V2 */
    // let (tx, mut rx): (UnboundedSender<usize>, UnboundedReceiver<usize>) = unbounded();

    // let sender = thread::spawn(move || {
    //     println!("sleep");
    //     thread::sleep(time::Duration::from_secs(sleep_sec));
    //     tx.unbounded_send(1).expect("Unable to send on channel");
    // });

    // let counter = 0;
    // // let mut notreadise = rxes;
    // loop {
    //     println!("loop start");
    //     let counter = counter + 1;
    //     match rx.by_ref().collect().poll() {
    //         Ok(Async::NotReady) => {
    //             println!("Async::NotReady: {}", counter);
    //         },
    //         _ => {
    //             println!("loop.other: {}", counter);
    //             break;
    //         },
    //     }
    // };

    // sender.join().expect("The sender thread has panicked");


    // let fut = match receiver.by_ref().collect().poll() {



    // let fut = match receiver.poll() {
    //     Async::Ready(items_vec) => Some(1),
    //     _ => return None
    // }


    /** V3 */
    // let (tx, rx) = mpsc::channel();

    // for i in 0..n {
    //     let tx = tx.clone();

    //     thread::spawn(move || {
    //         thread::sleep(time::Duration::from_secs(sleep_sec));
    //         let answer = i * i;

    //         tx.send(answer).unwrap();
    //     });
    // }

    // for _ in 0..n {
    //     println!("{}", rx.recv().unwrap());
    // }


    /** V4 */
    let mut txes = Vec::<(Sender<usize>, usize)>::new();
    let mut rxes = Vec::<Receiver<usize>>::new();

    (0..n).for_each(|i| {
        println!("tasks: {}", i);
        let (tx, rx) = mpsc::channel();
        txes.push((tx, i));
        rxes.push(rx);
    });

    let mut jhs = Vec::<thread::JoinHandle<_>>::new();
    while let Some((tx, i)) = txes.pop() {
        let jh = thread::spawn(move || {
            println!("sleep: {}", i);
            thread::sleep(time::Duration::from_secs(sleep_sec));
            tx.send(i.clone()).expect("Unable to send on channel");
        });
        jhs.push(jh);
    };
    // let senders = txes.into_iter().map(|(tx, i)| {
    //     thread::spawn(move || {
    //         println!("sleep: {}", i);
    //         thread::sleep(time::Duration::from_secs(sleep_sec));
    //         tx.send(i.clone()).expect("Unable to send on channel");
    //     })
    // });

    let f = rxes.iter().map(|rx| {rx.recv().unwrap()}).fold(0, |sum, i| sum + i);
    jhs.into_iter().for_each(|jh| {jh.join().expect("The sender thread has panicked");});

    println!("end: {:?}", f);
}
