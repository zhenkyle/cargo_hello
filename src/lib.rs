use std::thread::JoinHandle;

pub struct ThreadPool {
    thread: JoinHandle<()>,
}

impl ThreadPool {
    pub fn new() -> ThreadPool {
        let thread = std::thread::spawn(|| {});
        ThreadPool {thread: thread}
    }

    pub fn execute<F>(&self, f: F)
    where
        F: FnOnce(),
        F: Send + 'static,
    {
    }
}
