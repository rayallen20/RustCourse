use std::sync::{Arc, mpsc, Mutex};
use std::thread;
use crate::pool::Message;

pub(crate) struct Worker {
    pub(crate) id: usize,
    pub(crate) thread: Option<thread::JoinHandle<()>>
}

impl Worker {
    pub(crate) fn new(id: usize, receiver: Arc<Mutex<mpsc::Receiver<Message>>>) -> Worker {
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
            thread: Some(thread)
        }
    }
}