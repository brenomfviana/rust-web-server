use std::sync::Arc;
use std::sync::Mutex;
use std::thread;
use std::sync::mpsc;

/// Job to be performed by the workers.
type Job = Box<dyn FnOnce() + Send + 'static>;

/// Thread pool of the server.
pub struct ThreadPool {
  workers: Vec<Worker>,
  sender: mpsc::Sender<Job>,
}

impl ThreadPool {
  /// Create a new ThreadPool.
  ///
  /// The size is the number of threads in the pool.
  ///
  /// # Panics
  ///
  /// The `new` function will panic if the size is zero.
  pub fn new(size: usize) -> ThreadPool {
    // Check if the therad pool size is zero
    assert!(size > 0);
    // Create a channel
    let (sender, receiver) = mpsc::channel();
    // Turn the receiver into a mutex and create a shared reference to it
    let receiver = Arc::new(Mutex::new(receiver));
    // Create a vector of workers
    let mut workers = Vec::with_capacity(size);
    // Create all `size` workers
    for id in 0..size { workers.push(Worker::new(id, Arc::clone(&receiver))); }
    // Return a new thread pool
    ThreadPool { workers, sender }
  }

  /// Runs the thread pool.
  pub fn execute<F>(&self, f: F) where F: FnOnce() + Send + 'static {
    // Turn the function into a job
    let job = Box::new(f);
    // Create the sender with the job
    self.sender.send(job).unwrap();
  }
}

/// Worker of the server.
struct Worker {
  id: usize,
  thread: thread::JoinHandle<()>,
}

impl Worker {
  /// Creates a new worker with the ID `id`.
  fn new(id: usize, receiver: Arc<Mutex<mpsc::Receiver<Job>>>) -> Worker {
    // Spwan the worker's thread that runs receiver.
    let thread = thread::spawn(move || loop {
      // Get the job
      let job = receiver.lock().unwrap().recv().unwrap();
      // Print log
      println!("Worker {} got a job; executing.", id);
      // Perform the job
      job();
    });
    // Return the new worker
    Worker { id, thread }
  }
}
