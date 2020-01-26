use std::thread;
use std::sync::mpsc;
use std::sync::{Mutex, Arc}; 

type Job = Box<dyn FnOnce() + Send + 'static>;

enum Message {
    NewJob(Job),
    Terminate,
}

struct Worker {
    id: usize,
    thread: Option<thread::JoinHandle<()>>,
}

impl Worker {
    fn new(id: usize, receiver: Arc<Mutex<mpsc::Receiver<Message>>>) -> Worker {
            let thread = thread::spawn(move || {
                loop {

                    let message = receiver.lock().unwrap().recv().unwrap();
                    match message {
                        Message::NewJob(job) => {
                            println!("Worker {} got a job; executing.", id);
                            job();
                        },
                        Message::Terminate => {
                            println!("Worker {} was told to terminate.", id);
                            break;
                        }
                        
                    }
                }
            });
        
        Worker {
            id,
            thread: Some(thread),
        }
    }
}

pub struct ThreadPool {
    workers: Vec<Worker>,
    sender: mpsc::Sender<Message>,
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
        self.sender.send(Message::NewJob(job)).unwrap();
    }
}

impl Drop for ThreadPool {
    fn drop(&mut self) {
        println!("capacity:{}", self.workers.capacity());
        for _i in 0..self.workers.capacity() {
            self.sender.send(Message::Terminate).unwrap();
        }

        for worker in &mut self.workers {
            println!("Shutting down worker {}", worker.id);

            if let Some(thread) = worker.thread.take() {
                thread.join().unwrap();
            }
        }
    }
}
