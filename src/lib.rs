use std::thread;
use std::sync::mpsc;
use std::sync::{Mutex, Arc}; 

type Job = Box<dyn FnOnce() + Send + 'static>;

struct Worker {
    id: usize,
    thread: thread::JoinHandle<()>,
}

impl Worker {
    fn new(id: usize, thread: thread::JoinHandle<()>) -> Worker {
        Worker {
            id,
            thread
        }
    }
}

pub struct ThreadPool {
    workers: Vec<Worker>,
    sender: mpsc::Sender<Job>,
}

impl ThreadPool {
    pub fn new(size: usize) -> ThreadPool {
        assert!(size > 0);
        let (sender, receiver) = mpsc::channel();
        let receiver = Mutex::new(receiver);
        let receiver = Arc::new(receiver);
        let mut workers = Vec::with_capacity(4);
        for i in 0..size {
            let receiver = Arc::clone(&receiver);
            let thread = thread::spawn(move || {
                loop {
                    let job: Job;
                    job = receiver.lock().unwrap().recv().unwrap();
                    println!("Got a job!");
                    job();
                }
            });
            workers.push(Worker::new(i, thread));
        }

        ThreadPool {
            workers,
            sender
        }
    }

    pub fn execute<F>(&self, f: F)
    where
        F: FnOnce(),
        F: Send + 'static,
    {
        let job = Box::new(f);
        self.sender.send(job).unwrap();
    }
}
