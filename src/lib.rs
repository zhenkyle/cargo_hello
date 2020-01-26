use std::thread;
use std::sync::mpsc;
use std::sync::{Mutex, Arc}; 

type Job = Box<dyn FnOnce() + Send + 'static>;

struct Worker {
    id: usize,
    thread: thread::JoinHandle<()>,
}

impl Worker {
    fn new(id: usize, receiver: Arc<Mutex<mpsc::Receiver<Job>>>) -> Worker {
            let thread = thread::spawn(move || {
                loop {

                    let job = receiver.lock().unwrap().recv().unwrap();
                    println!("Worker {} got a job; executing.", id);
                    job();
                }
            });
        
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
            workers.push(Worker::new(i, Arc::clone(&receiver)));
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

impl Drop for ThreadPool {
    fn drop(&mut self) {
//        for i in 0..self.workers.capacity() {
//            self.sender.send(Terminat).unwrap();
//        }

        for worker in self.workers.iter() {
            worker.thread.join();
        }
    }
}
