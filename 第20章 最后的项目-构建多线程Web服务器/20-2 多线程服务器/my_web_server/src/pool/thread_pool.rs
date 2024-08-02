use std::sync::{Arc, mpsc, Mutex};
use crate::pool::Job;
use crate::pool::Worker;

pub struct ThreadPool {
    workers: Vec<Worker>,
    sender: mpsc::Sender<Job>
}

impl ThreadPool {
    /// 创建一个新的线程池
    /// size: 线程池中线程的数量
    /// # Panics
    /// 关联`new()`在`size`为0时会触发panic
    pub fn new(size: usize) -> ThreadPool {
        assert!(size > 0);
        let (sender, receiver) = mpsc::channel();
        let receiver = Arc::new(Mutex::new(receiver));
        let mut workers = Vec::with_capacity(size);

        for id in 0..size {
            workers.push(Worker::new(id, receiver.clone()));
        }

        ThreadPool {
            workers,
            sender
        }
    }

    pub fn execute<F>(&self, f :F)
    where
        F: FnOnce() + Send + 'static
    {
        let job = Box::new(f);
        self.sender.send(job).unwrap()
    }
}