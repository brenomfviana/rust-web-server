use std::sync::Arc;
use std::sync::Mutex;
use std::thread;
use std::sync::mpsc;

/// Job to be performed by the workers.
type Job = Box<dyn FnOnce() + Send + 'static>;

/// Server message.
enum Message {
  NewJob(Job),
  Terminate,
}

/// Thread pool of the server.
pub struct ThreadPool {
  workers: Vec<Worker>,
  sender: mpsc::Sender<Message>,
}

impl Drop for ThreadPool {
  /// Shutdown all workers when the job is done.
  fn drop(&mut self) {
    // Print generatl shutdown log
    println!("Sending terminate message to all workers.");
    // Notify all workers about the shutdown
    for _ in &self.workers { self.sender.send(Message::Terminate).unwrap(); }
    // Shutdown all workers
    for worker in &mut self.workers {
      // Print shutdown log
      println!("Shutting down worker {}", worker.id);
      // Shutdown the worker
      if let Some(thread) = worker.thread.take() { thread.join().unwrap(); }
    }
  }
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

  /// Run the thread pool.
  ///
  /// The job to be performed by the workers.
  ///
  /// # Panics
  ///
  /// The `execute` function will panic if the data will never be received.
  pub fn execute<F>(&self, f: F) where F: FnOnce() + Send + 'static {
    // Turn the function into a job
    let job = Box::new(f);
    // Create the sender with the job
    self.sender.send(Message::NewJob(job)).unwrap();
  }
}

/// Worker of the server.
struct Worker {
  id: usize,
  thread: Option<thread::JoinHandle<()>>,
}

impl Worker {
  /// Create a new worker with the ID `id`.
  fn new(id: usize, receiver: Arc<Mutex<mpsc::Receiver<Message>>>) -> Worker {
    // Spwan the worker's thread that runs receiver.
    let thread = Some(thread::spawn(move || loop {
      // Get the reveiver message
      let message = receiver.lock().unwrap().recv().unwrap();
      // Check reveiver message
      match message {
        // Start the worker's job
        Message::NewJob(job) => {
          println!("Worker {} got a job; executing.", id);
          job();
        }
        // Stop the worker's job
        Message::Terminate => {
          println!("Worker {} was told to terminate.", id);
          break;
        }
      }
    }));
    // Return the new worker
    Worker { id, thread }
  }
}
