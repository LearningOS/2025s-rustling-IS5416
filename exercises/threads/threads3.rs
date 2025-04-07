// threads3.rs
//
// Execute `rustlings hint threads3` or use the `hint` watch subcommand
// for a hint
use std::sync::{mpsc, Arc, Mutex};
use std::thread;
use std::time::Duration;

struct Queue {
    length: u32,
    first_half: Vec<u32>,
    second_half: Vec<u32>,
}

impl Queue {
    fn new() -> Self {
        Queue {
            length: 10,
            first_half: vec![1, 2, 3, 4, 5],
            second_half: vec![6, 7, 8, 9, 10],
        }
    }
}

fn send_tx(q: Queue, tx: mpsc::Sender<u32>) -> () {
    let first_half = q.first_half.clone();
    let second_half = q.second_half.clone();

    // 将 tx 包裹在 Arc<Mutex<_>> 中
    let tx = Arc::new(Mutex::new(tx));

    // 第一个线程
    let tx1 = Arc::clone(&tx);
    thread::spawn(move || {
        for val in first_half {
            println!("sending {:?}", val);
            let tx = tx1.lock().unwrap(); // 获取锁
            tx.send(val).unwrap();
            thread::sleep(Duration::from_secs(1));
        }
    });

    // 第二个线程
    let tx2 = Arc::clone(&tx);
    thread::spawn(move || {
        for val in second_half {
            println!("sending {:?}", val);
            let tx = tx2.lock().unwrap(); // 获取锁
            tx.send(val).unwrap();
            thread::sleep(Duration::from_secs(1));
        }
    });
}

fn main() {
    let (tx, rx) = mpsc::channel();
    let queue = Queue::new();
    let queue_length = queue.length;

    send_tx(queue, tx);

    let mut total_received: u32 = 0;
    for received in rx {
        println!("Got: {}", received);
        total_received += 1;
    }

    println!("total numbers received: {}", total_received);
    assert_eq!(total_received, queue_length);
}
