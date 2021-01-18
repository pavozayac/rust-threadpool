use std::thread;
use std::collections::VecDeque;
use std::sync::mpsc;

struct ThreadPool {
    size:  usize,
    queue: VecDeque<Box<dyn FnOnce() + Send + 'static>>,
    handles: VecDeque<thread::JoinHandle<()>>
}

impl ThreadPool {
    fn new (size: usize) -> ThreadPool {
        assert!(size > 0);
        ThreadPool {
            size,
            queue: VecDeque::new(),
            handles: VecDeque::new()
        }
    }

    fn execute <F> (&mut self, closure: F) 
    where 
        F: FnOnce(),
        F: Send + 'static
    {
        if self.handles.len() <= self.size {
            self.handles.push_back(thread::spawn(closure));
        } else {
            self.queue.push_back(Box::new(closure));
        }
    }

    fn join (&mut self) {
        if self.queue.is_empty() {
            println!("Branch 1 for 1");
            for _ in 0..self.size {
                let t = self.handles.pop_front();
                if t.is_some(){
                    t.unwrap().join().expect("Could not join thread!");
                }
            }
        } else {
            for _ in 0..self.size {
                println!("Branch 2 for 1");
                let t = self.handles.pop_front();
                if t.is_some(){
                    t.unwrap().join().expect("Could not join thread!");
                }
            }

            for _ in 0..self.size {
                println!("Branch 2 for 2");

                let q = self.queue.pop_front();
                if q.is_some() {
                    self.handles.push_back(thread::spawn(q.unwrap()));
                }
            }
            self.join();
        }
    }
}

fn main() {
    println!("{}", std::process::id());

    std::thread::sleep(std::time::Duration::new(5, 0));

    let (tx, rx) = mpsc::channel();

    let mut pool = ThreadPool::new(4);

    for i in 0..20 {
        let t = tx.clone();
        pool.execute(move || {
            t.send(i).expect("Could not send message!");
        })
    }

    drop(tx);

    pool.join();

    for r in rx {
        println!("{}", r);
    }

}
