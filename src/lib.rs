use std::{
    sync::{mpsc, Arc, Mutex},
    thread,
};

pub struct ThreadPool {
    workers: Vec<Worker>,
    sender: mpsc::Sender<Job>,
}

impl ThreadPool {
    /// Create a new ThreadPool.
    ///
    /// The capacity is the number of threads in the pool.
    ///
    /// # Panics
    ///
    /// The `new` function will panic if the capacity is zero.
    pub fn new(capacity: u8) -> Self {
        assert!(capacity > 0);

        let (sender, receiver) = mpsc::channel();

        let receiver = Arc::new(Mutex::new(receiver));

        let mut workers = Vec::with_capacity(capacity as usize);

        for i in 0..capacity {
            // thread::spawn => crea y ejecuta el hilo
            // workers.push(thread::spawn(|| {}))
            workers.push(Worker::new(i, Arc::clone(&receiver)))
        }

        Self { workers, sender }
    }

    /// Build a new ThreadPool.
    ///
    /// The capacity is the number of threads in the pool.
    ///
    /// # Errors
    ///
    /// Returns a `PoolCreationError` if the capacity is zero.
    pub fn build(capacity: u8) -> Result<ThreadPool, PoolCreationError> {
        if capacity == 0 {
            return Err(PoolCreationError::ZeroCapacity);
        }

        let (sender, receiver) = mpsc::channel();

        let receiver = Arc::new(Mutex::new(receiver));

        let mut workers = Vec::with_capacity(capacity as usize);

        for i in 0..capacity {
            // despues implementaremos los thread
            workers.push(Worker::new(i, Arc::clone(&receiver)))
        }

        Ok(Self { workers, sender })
    }

    pub fn execute<F>(&self, f: F)
    where
        // FnOnce => porque el hilo para ejecutar una solicitud solo ejecutará el closure de esa solicitud una vez
        // Send => porque necesitamos transferir el closure de un hilo a otro
        // 'static => porque no sabemos cuanto tiempo tomara el thread en ejecutarse
        F: FnOnce() + Send + 'static,
    {
        let job: Job = Box::new(f);

        self.sender.send(job).unwrap();
    }
}

#[derive(Debug)]
pub enum PoolCreationError {
    ZeroCapacity,
}

struct Worker {
    id: u8,
    // thread: Option<thread::JoinHandle<()>>,
    thread: thread::JoinHandle<()>,
    // receiver: Arc<Mutex<mpsc::Receiver<Job>>>,
}

impl Worker {
    fn new(id: u8, receiver: Arc<Mutex<mpsc::Receiver<Job>>>) -> Worker {
        let thread = thread::spawn(move || loop {
            let job = receiver.lock().unwrap().recv().unwrap();
            println!("Worker {} got a job; executing.", id);
            job();
        });
        Worker { id, thread }
    }

    fn execute<F>(&mut self, f: F)
    where
        F: FnOnce() + Send + 'static,
    {
        // assert!(self.thread.is_none());

        // self.thread = Some(thread::spawn(f));
    }
}

type Job = Box<dyn FnOnce() + Send + 'static>;
