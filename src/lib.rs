use std::{
    sync::{mpsc, Arc, Mutex},
    thread,
};

pub struct ThreadPool {
    workers: Vec<Worker>,
    sender: Option<mpsc::Sender<Job>>,
}

type Job = Box<dyn FnOnce() + Send + 'static>;

impl ThreadPool {
    /// Create a new ThreadPool.
    ///
    /// The capacity is the number of threads in the pool.
    ///
    /// # Panics
    ///
    /// The `new` function will panic if the capacity is zero.
    pub fn new(capacity: usize) -> Self {
        assert!(capacity > 0);

        let (sender, receiver) = mpsc::channel();
        let receiver = Arc::new(Mutex::new(receiver));

        let mut workers = Vec::with_capacity(capacity);
        for id in 0..capacity {
            workers.push(Worker::new(id, Arc::clone(&receiver)));
        }

        Self {
            workers,
            sender: Some(sender),
        }
    }

    /// Build a new ThreadPool.
    ///
    /// The capacity is the number of threads in the pool.
    ///
    /// # Errors
    ///
    /// Returns a `PoolCreationError` if the capacity is zero.
    pub fn build(capacity: usize) -> Result<Self, PoolCreationError> {
        if capacity == 0 {
            return Err(PoolCreationError::ZeroCapacity);
        }

        let (sender, receiver) = mpsc::channel();
        let receiver = Arc::new(Mutex::new(receiver));

        let mut workers = Vec::with_capacity(capacity);
        for id in 0..capacity {
            workers.push(Worker::new(id, Arc::clone(&receiver)))
        }

        Ok(Self {
            workers,
            sender: Some(sender),
        })
    }

    pub fn execute<F>(&self, f: F)
    where
        F: FnOnce() + Send + 'static,
    {
        let job = Box::new(f);
        self.sender.as_ref().unwrap().send(job).unwrap();
    }
}

impl Drop for ThreadPool {
    fn drop(&mut self) {
        drop(self.sender.take());

        for worker in &mut self.workers {
            println!("Shutting down worker {}", worker.id);

            if let Some(thread) = worker.thread.take() {
                thread.join().unwrap();
            }
        }
    }
}
struct Worker {
    id: usize,
    thread: Option<thread::JoinHandle<()>>,
}

impl Worker {
    fn new(id: usize, receiver: Arc<Mutex<mpsc::Receiver<Job>>>) -> Self {
        let thread = thread::spawn(move || loop {
            let message = receiver.lock().unwrap().recv();
            match message {
                Ok(job) => {
                    println!("Worker {id} got a job; executing.");
                    job();
                }
                Err(_) => {
                    println!("Worker {id} disconnected; shutting down.");
                    break;
                }
            }
        });
        Self {
            id,
            thread: Some(thread),
        }
    }
}

#[derive(Debug)]
pub enum PoolCreationError {
    ZeroCapacity,
}
