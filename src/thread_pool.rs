use std::sync::{Arc, Mutex};
use crate::{message, worker::{self, Worker}};
use concurrent_queue::ConcurrentQueue;

pub struct ThreadPool {
    workers: Vec<worker::Worker>,
    tasks:Arc<Mutex<ConcurrentQueue<message::Message>>>
}

impl ThreadPool {

    pub fn new(n_threads: usize) -> Self {

        assert!(n_threads > 0);

        let tasks: Arc<Mutex<ConcurrentQueue<message::Message>>> = Arc::new(Mutex::new(ConcurrentQueue::unbounded()));

        let mut workers = Vec::with_capacity(n_threads);
        
        for id in 0..n_threads {
            
            let work_thread = Worker::new(id, Arc::clone(&tasks));    
            workers.push(work_thread);
           
        }

        Self {
            workers,
            tasks
        }
    }

    pub fn spawn<F>(&self, f: F)
        where
            F: FnOnce() + Send + 'static
    {
        let job = Box::new(f);

        self.tasks.lock().unwrap().push(message::Message::NewJob(job));
    }

}

impl Drop for ThreadPool {
    fn drop(&mut self) {
        println!("Sending terminate message to all workers.");

        for _ in &mut self.workers {
            self.tasks.lock().unwrap().push(message::Message::Terminate);
        }

        println!("Shutting down all workers.");

        for worker in &mut self.workers {
            println!("Shutting down worker {}", worker.id);

            if let Some(thread) = worker.thread.take() {
                thread.join();
            //    let result = 
            //    match result {
            //        Ok(i) => {},
            //        _ => {}
            //    }

            }
        }
    }
}


