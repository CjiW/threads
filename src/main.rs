use std::{
    sync::{
        mpsc::{channel, Receiver, Sender},
        Arc, Mutex,
    },
    thread::{self, sleep, JoinHandle}, time,
};

fn foo(i: usize) {
    let t = (i % 3 + 1) as u64;
    println!("task {} start sleep for {}s...", i, t);
    sleep(time::Duration::from_secs(t));
    println!("task {} done.", i);
}

fn main() {
    let tp = ThreadPool::new(4);
    for i in 1..=10 {
        tp.execute(move || {
            foo(i)
        })
    }
    println!();
    let mut i = 0;
    loop {
        println!("====== tick {} =======", i);
        sleep(time::Duration::from_secs(1));
        println!("=====================");
        i+=1;
    }
}

type Task = Box<dyn FnOnce() + Send + 'static>;

pub struct ThreadPool {
    works: Vec<Worker>,
    sender: Sender<Task>,
}

impl ThreadPool {
    fn new(size: usize) -> ThreadPool {
        let mut works = Vec::with_capacity(size);
        let (sender, receiver) = channel();
        let receiver = Arc::new(Mutex::new(receiver));
        for i in 0..size {
            works.push(Worker::new(i, Arc::clone(&receiver)));
        }
        ThreadPool { works, sender }
    }

    pub fn execute<F>(&self, f: F)
    where
        F: FnOnce() + Send + 'static,
    {
        self.sender.send(Box::new(f)).unwrap();
    }
}

struct Worker {
    id: usize,
    thread: JoinHandle<()>,
}

impl Worker {
    fn new(id: usize, recvier: Arc<Mutex<Receiver<Task>>>) -> Worker {
        Worker {
            id,
            thread: thread::spawn(move || loop {
                let t = recvier.lock().unwrap().recv().unwrap();
                println!("worker {} recv a Task.", id);
                t();
            }),
        }
    }
}
