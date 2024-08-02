use std::sync::{Arc, mpsc, Mutex};
use crate::pool::Message;
use crate::pool::Worker;

pub struct ThreadPool {
    workers: Vec<Worker>,
    sender: mpsc::Sender<Message>
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
        self.sender.send(Message::NewJob(job)).unwrap()
    }
}

impl Drop for ThreadPool {
    fn drop(&mut self) {
        // 发送终止信号给每个线程
        for _ in &mut self.workers {
            self.sender.send(Message::Terminate).unwrap()
        }

        // 等待每个线程终止
        for worker in &mut self.workers {
            println!("Shutting down worker {}", worker.id);
            if let Some(thread) = worker.thread.take() {
                thread.join().unwrap();
            }
        }

        // 注意: 这里不能1次在遍历中既发送终止信号又等待线程终止
        // 因为主线程是不确定发出的这个终止信号究竟被哪个子线程接收到了
        // 这里我们假设有2个worker(worker1和worker2),主线程在遍历时,先遍历到worker1,
        // 发送了1个终止信号.但此时可能worker1正在执行任务(正在match表达式的NewJob(job)这个分支中),
        // 没有接收到终止信号,而此时恰巧worker2处于空闲,接收到了终止信号
        // 而此时主线程的join()操作中,等待的是worker1的线程,而worker1的线程因为没有收到终止信号,
        // 即使它完成了工作,也不会终止.
        // 因此主线程永远无法发送第2个终止信号,也就无法终止worker1的线程.这就造成了死锁!
        // for worker in &mut self.workers {
        //     // 发送终止信号
        //     self.sender.send(Message::Terminate).unwrap();
        //
        //     // 等待线程终止
        //     println!("Shutting down worker {}", worker.id);
        //     if let Some(thread) = worker.thread.take() {
        //         thread.join().unwrap();
        //     }
        // }
    }
}