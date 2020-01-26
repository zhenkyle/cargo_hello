use std::thread;
use std::sync::mpsc;
use std::sync::{Mutex, Arc}; 

pub struct ThreadPool {
    threads: Vec<thread::JoinHandle<()>>,
    sender: std::sync::mpsc::Sender<u32>,
}

impl ThreadPool {
    pub fn new(size: usize) -> ThreadPool {
        assert!(size > 0);
        let (sender, receiver) = mpsc::channel();
        let receiver = Mutex::new(receiver);
        let receiver = Arc::new(receiver);
        let mut threads = Vec::with_capacity(4);
        for i in 0..size {
            let receiver = Arc::clone(&receiver);
            let thread = std::thread::spawn(move || {
                loop {
                    let num = receiver.lock().unwrap().recv().unwrap();
                    println!("Got {}!", num);
                }
            });
            threads.push(thread);
        }

        ThreadPool {
            threads,
            sender
        }
    }

    pub fn execute<F>(&self, f: F)
    where
        F: FnOnce(),
        F: Send + 'static,
    {
        self.sender.send(5).unwrap();
    }
}
