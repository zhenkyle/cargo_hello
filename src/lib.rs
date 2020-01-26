use std::thread::JoinHandle;

pub struct ThreadPool {
    threads: Vec<JoinHandle<()>>,
}

impl ThreadPool {
    pub fn new() -> ThreadPool {
        let mut threads = Vec::with_capacity(4);
        for i in 0..4 {
            let thread = std::thread::spawn(|| {});
            threads.push(thread);
        }
        ThreadPool {threads}
    }

    pub fn execute<F>(&self, f: F)
    where
        F: FnOnce(),
        F: Send + 'static,
    {
    }
}
