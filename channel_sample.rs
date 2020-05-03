// Multi-producer, single-consumer FIFO queue communication primitives.

// This module provides message-based communication over channels, concretely defined among three types:

// Sender
// SyncSender
// Receiver
// A Sender or SyncSender is used to send data to a Receiver. Both senders are clone-able (multi-producer) such that many threads can send simultaneously to one receiver (single-consumer).

// These channels come in two flavors:

// An asynchronous, infinitely buffered channel. The channel function will return a (Sender, Receiver) tuple where all sends will be asynchronous (they never block). The channel conceptually has an infinite buffer.

// A synchronous, bounded channel. The sync_channel function will return a (SyncSender, Receiver) tuple where the storage for pending messages is a pre-allocated buffer of a fixed size. All sends will be synchronous by blocking until there is buffer space available. Note that a bound of 0 is allowed, causing the channel to become a "rendezvous" channel where each sender atomically hands off a message to a receiver.

// Simple usuage

use std::thread;
use std::sync::mpsc::{Sender, Receiver};
use std::sync::mpsc::channel;
use std::sync::mpsc::sync_channel;

fn main()
{
    println!("asynchronous channels");

    // Create a simple streaming channel
    let (tx, rx) = channel();

    thread::spawn(move|| {
        tx.send(10).unwrap();
    });
    
    println!("{}", rx.recv().unwrap());

    //Shared usage:

    let (tx, rx) = channel();
    for i in 0..10 {
        let tx = tx.clone();
        thread::spawn(move|| {
            tx.send(i).unwrap();
        });
    }

    for _ in 0..10 {
        let j = rx.recv().unwrap();
        println!("{}", j);
    }

    println!("synchronous channels");

    //Synchronous channels:

    let (tx, rx) = sync_channel::<i32>(0);

    for i in 0..10 {
        let tx = tx.clone();
        thread::spawn(move|| {
            tx.send(i).unwrap();
        });
    }

    for _ in 0..10 {
        let j = rx.recv().unwrap();
        println!("{}", j);
    }
}
