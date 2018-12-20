use std::{thread, time};
use std::sync::mpsc::{self, Sender, Receiver};

pub fn n_tasks(n: usize, sleep_sec: u64) {
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

    let f = rxes.iter().map(|rx| {rx.recv().unwrap()}).fold(0, |sum, i| sum + i);
    jhs.into_iter().for_each(|jh| {jh.join().expect("The sender thread has panicked");});

    println!("end: {:?}", f);
}

pub fn channel_recv(n: usize, sleep_sec: u64) {
    let mut txes = Vec::<(Sender<usize>, usize)>::new();
    let mut rxes = Vec::<Receiver<usize>>::new();

    (0..n).for_each(|i| {
        println!("tasks: {}", i);
        let (tx, rx) = mpsc::channel();
        txes.push((tx, i));
        rxes.push(rx);
    });

    for rx in rxes {
        rx.recv().unwrap();
    };
    println!("end!");
}
