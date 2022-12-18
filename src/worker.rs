use std::sync::{Arc, Mutex};
use std::thread;
use std::time::Duration;

// custom threadpool implementation
// may be replaced by crate, if I find the right one. Have looked at crossbeam and rayon.
// Rayon par_iter for instance doesn't seem usable, because it lacks buffering, which I guess would increase context switching
// this is in principle an uncorroborated thought
// It does use crossbeam_channel::bounded, which is crucial to avoid memory overload (more than 30 Gbs instead of at most ~3 in the current implementation)
enum Message {
    NewJob(Job),
    Terminate,
}

pub struct ThreadPool {
    workers: Vec<Worker>,
    sender: crossbeam_channel::Sender<Message>,
}

type Job = Box<dyn FnOnce() + Send + 'static>;

impl ThreadPool {
    /// Create a new threadpool
    /// size: the number of threads in the pool
    /// # Panics
    #[must_use = "size must be >0"]
    pub fn new(size: usize) -> Self {
        assert!(size > 0);

        // The max number of messages in the channel should be higher as the number of workerthreads grows
        // to avoid idle threads.
        let (sender, receiver) = crossbeam_channel::bounded(size * 2); // just a stab
        let receiver = Arc::new(Mutex::new(receiver));
        let mut workers = Vec::with_capacity(size);

        for _ in 0..size {
            workers.push(Worker::new(Arc::clone(&receiver)));
        }

        Self {
            workers,
            sender,
        }
    }

    /// submit a function for multithreaded processing
    pub fn execute<F>(&self, f: F) where F: FnOnce() + Send + 'static {
        let job = Box::new(f);

        self.sender.send(Message::NewJob(job)).unwrap();
    }

    /// wait for workerthreads to finish
    pub fn wait(&mut self) {
        for worker in &mut self.workers {
            self.sender.send(Message::Terminate).unwrap_or(());
            if let Some(thread) = worker.thread.take() {
                thread.join().unwrap();
            }
        }
    }
}

struct Worker {
    thread: Option<thread::JoinHandle<()>>,
}

impl Worker {
    fn new(receiver: Arc<Mutex<crossbeam_channel::Receiver<Message>>>) -> Self {
        let thread = thread::spawn(move || {
            loop {
                // idle worker threads die quickly, so avoid this situation
                // the timeout can be increased if that is not what you want
                // For small workloads the timeout is necessary, because some theads may never get the Terminate command and thus prevent the process from ending.
                let message_r = receiver.lock().unwrap().recv_timeout(Duration::from_secs(1));
                if let Ok(message) = message_r {
                    match message {
                        Message::NewJob(job) => {
                            job();
                        }
                        Message::Terminate => {
                            break;
                        }
                    }
                } else {
                    break;
                }
            }
        });

        Self {
            thread: Some(thread),
        }
    }
}