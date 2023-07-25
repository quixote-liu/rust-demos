use std::sync::mpsc;
use std::thread;
use std::time::Duration;

pub fn run() {
    let handler = thread::spawn( ||
        for i in 10..20 {
            println!("the num in the thread is {}", i);
            thread::sleep(Duration::from_millis(1));
        }
    );

    for i in 1..5 {
        println!("the num in the main is {}", i);
        thread::sleep(Duration::from_millis(1));
    }

    handler.join().unwrap();
}

pub fn run_move() {
    let v  = vec![1, 2, 3];
    
    let handler = thread::spawn(move || {
        println!("the value is {:?}", v);
    });

    handler.join().unwrap();
}

pub fn run_mpsc() {
    let (tx, rx) = mpsc::channel();

    // 生产者1
    let tx1 = tx.clone();
    thread::spawn(move || {
        let values = vec![
            String::from("t1_a"),
            String::from("t1_b"),
            String::from("t1_c"),
            String::from("t1_d"),
        ];
        for val in values {
            tx1.send(val).unwrap();
            thread::sleep(Duration::from_secs(1));
        }
        let end_msg = String::from("End_t1");
        tx1.send(end_msg).unwrap();
    });
    
    thread::spawn(move || {
        let values = vec!{
            String::from("t2_a"),
            String::from("t2_b"),
            String::from("t2_c"),
            String::from("t2_d"),
        };
        for val in values {
            tx.send(val).unwrap();
            thread::sleep(Duration::from_secs(1));
        }
        let val = String::from("End_t2");
        tx.send(val).unwrap();
    });

    for received in rx {
        println!("received = {}", received);
    }
}