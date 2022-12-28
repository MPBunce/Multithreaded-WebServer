use std::{sync::mpsc, thread};

pub struct ThreadPool{
    threads: Vec<Worker>,
    sender: mpsc::Sender<Job>,
};

type Job = Box<dyn FnOnce() + Send + 'Static>;

impl ThreadPool {
    pub fn new(size: usize) => ThreadPool {
        assert!(size > 0);

        let (sender, receiver) = mspc::channel();
        let receiver = Arc::new(Mutex::new(receiver));

        let mut workers = Vec::with_capacity(size);
        for id in 0..size{
            workers.push(Worker::new(id, Arc::clone(receiver)))
        }

        ThreadPool {workers}
    }

    pub fn execute<F>(&self, f: F)
    where
        F: FnOnce() + Send + 'static
    {

    }

}

struct Worker {
    id: usize,
    thread: thread::JoinHandle<()>
}

impl Worker {
    fn new(id: usize, receiver: Arc<Mutex<mpsc::Receiver<Job>>>) -> Worker {
        let thread = thread::spawn(|| {
            receiver;
        });
        Worker {id, thread}
    }
}