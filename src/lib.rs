pub struct ThreadPool {
}

impl ThreadPool {
    pub fn new() -> ThreadPool {
        ThreadPool {}
    }

    pub fn execute<F>(&self, f: F)
    where
        F: FnOnce(),
        F: Send + 'static,
    {
    }
}
