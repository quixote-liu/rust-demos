use std::sync::{Mutex, Arc};
use std::thread;

pub fn run() {
    let counter = Arc::new(Mutex::new(0));
    let mut handlers = vec![];

    for _ in 0..10 {
        let counter = Arc::clone(&counter);

        let handler = thread::spawn(move || {
            let mut num = counter.lock().unwrap();
            *num += 1;
        });

        handlers.push(handler);
    }

    for handler in handlers {
        handler.join().unwrap();
    }

    println!("the counter = {}", *counter.lock().unwrap());
}