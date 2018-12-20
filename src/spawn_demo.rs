use std::{thread, time};

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
