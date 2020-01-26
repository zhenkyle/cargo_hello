use std::thread::JoinHandle;
use std::sync::mpsc::channel;

pub struct ThreadPool {
    threads: Vec<JoinHandle<()>>,
    sender: std::sync::mpsc::Sender<()>,
}

impl ThreadPool {
    pub fn new() -> ThreadPool {
        let (sender, receiver) = channel();
        let mut threads = Vec::with_capacity(4);
        for i in 0..4 {
            let thread = std::thread::spawn(|| {
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
    }
}
