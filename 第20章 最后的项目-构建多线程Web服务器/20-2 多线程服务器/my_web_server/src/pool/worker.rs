use std::sync::{Arc, mpsc, Mutex};
use std::thread;
use crate::pool::Job;

pub(crate) struct Worker {
    id: usize,
    thread: thread::JoinHandle<()>
}

impl Worker {
    pub(crate) fn new(id: usize, receiver: Arc<Mutex<mpsc::Receiver<Job>>>) -> Worker {
        let thread = thread::spawn(move || {
            loop {
                let job = receiver.lock().unwrap().recv().unwrap();
                println!("Worker {} got a job; executing.", id);
                job();
            }

            // while let Ok(job) = receiver.lock().unwrap().recv() {
            //     println!("Worker {} got a job; executing.", id);
            //     job();
            // }
        });
        Worker {
            id,
            thread
        }
    }
}