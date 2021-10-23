
use std::thread;
use std::sync::{Arc, Mutex};
use crate::message::Message;
use concurrent_queue::ConcurrentQueue;

pub struct Worker {
    pub id: usize,
    pub thread: Option<thread::JoinHandle<()>>
}

impl Worker {

    pub fn new(id: usize, tasks: Arc<Mutex<ConcurrentQueue<Message>>>) -> Worker {

        let thread = thread::spawn(move || {
            
            loop {

                if tasks.lock().unwrap().is_empty() {
                    continue
                }

                let result = tasks.lock().unwrap().pop();
            
                match result {

                    Ok(Message::NewJob(job)) => {
                        println!("Worker {} got a job; executing.", id);

                        match job.call_box() {
                            _ => {},
                        };
                    },
                    Ok(Message::Terminate) => {
                        println!("Worker {} was told to terminate.", id );
                        break;
                    },
                    _ => {
                        println!("panic in worker {}!", id); 
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
