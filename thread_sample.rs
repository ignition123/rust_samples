use std::thread;
use std::sync::mpsc;
use std::time::Duration;
use std::sync::{Mutex, Arc};

const NUM_TIMERS: usize = 24;

fn timer(d: usize, tx: mpsc::Sender<usize>)
{
    thread::spawn(move || {
        println!("{}: setting timer...", d);
        thread::sleep(Duration::from_secs(d as u64));
        println!("{}: sent!", d);
        tx.send(d).unwrap();
    });    
}

fn main()
{

    // normal threads

    let mut c = vec![];

    for i in 0..10
    {
        c.push(thread::spawn(move || {
            println!("thread number {}", i);
        }));
    }

    for j in c
    {
        j.join();
    }

    // channels

    let (tx, rx) = mpsc::channel();

    thread::spawn(move || {
        tx.send(42).unwrap();
    });

    println!("got {}", rx.recv().unwrap());

    let (tx, rx) = mpsc::channel();

    for i in 0..NUM_TIMERS
    {
        timer(i, tx.clone());
    }

    for v in rx.iter().take(NUM_TIMERS)
    {
        println!("{}: received!", v);
    }

    // mutex and Arc

    let c = Arc::new(Mutex::new(0));

    let mut hs = vec![];

    for _ in 0..10
    {
        let c = Arc::clone(&c);
        let h = thread::spawn(move || {
            let mut num = c.lock().unwrap();
            *num += 1;
        });

        hs.push(h);
    }

    for h in hs
    {
        h.join().unwrap();
    }

    println!("Result: {}", *c.lock().unwrap());
}