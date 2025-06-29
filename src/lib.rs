use std::{
    thread,
    sync::{
        mpsc,
        Arc,
        Mutex
    },
};

type Job = Box<dyn FnOnce() + Send + 'static>;

pub struct Threadpool {
    workers: Vec<Worker>,
    sender: Option<mpsc::Sender<Job>>,
}

impl Threadpool {
    pub fn new(size: usize) -> Threadpool {
        assert!(size > 0);

        let mut workers = Vec::with_capacity(size);
        let (sender, receiver) = mpsc::channel();
        let receiver = Arc::new(Mutex::new(receiver));

        for id in 0..size {
            workers.push(Worker::new(id, Arc::clone(&receiver)));
        }

        Threadpool { 
            workers, 
            sender: Some(sender) 
        }
    }
    pub fn execute<F, T>(&self, f: F) -> mpsc::Receiver<T> 
        where F: FnOnce() -> T + Send + 'static,
        T: Send + 'static {
            let (_sender, _receiver) = mpsc::channel();
            let job = Box::new(move || {
               let result = f();
               _sender.send(result).unwrap();
            });
            self.sender.as_ref().unwrap().send(job).unwrap();
            _receiver
    }
}

impl Drop for Threadpool {
    fn drop(&mut self) {
        drop(self.sender.take());
        for worker in &mut self.workers {
            println!("Shutting down worker: {}", worker.id);
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
    fn new(id: usize, receiver: Arc<Mutex<mpsc::Receiver<Job>>>) -> Worker {
        let thread = thread::spawn(move || {
            loop {
                let message = receiver.lock().unwrap().recv();
                match message {
                    Ok(job) => {
                        job();
                    }
                    Err(_) => {
                        println!("Worker {id} is shutting down");
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
