// threads3.rs
//
// Execute `rustlings hint threads3` or use the `hint` watch subcommand for a
// hint.


use std::sync::mpsc;
use std::sync::{Arc, Mutex};
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

fn send_tx(q: Arc<Queue>, tx: Arc<Mutex<mpsc::Sender<u32>>>) -> () {
    let tx1 = Arc::clone(&tx);
    let tx2 = Arc::clone(&tx);

    let q1 = Arc::clone(&q);
    let q2 = Arc::clone(&q);

    thread::spawn(move || {
        for val in &q1.first_half {
            println!("sending {:?}", val);
            let tx_guard = tx1.lock().unwrap();
            tx_guard.send(*val).unwrap();
            thread::sleep(Duration::from_secs(1));
        }
    });

    thread::spawn(move || {
        for val in &q2.second_half {
            println!("sending {:?}", val);
            let tx_guard = tx2.lock().unwrap();
            tx_guard.send(*val).unwrap();
            thread::sleep(Duration::from_secs(1));
        }
    });
}

fn main() {
    let (tx, rx) = mpsc::channel();
    let queue = Arc::new(Queue::new());
    let queue_length = queue.length;

    send_tx(Arc::clone(&queue), Arc::new(Mutex::new(tx)));

    let mut total_received: u32 = 0;
    for received in rx {
        if received == 0 {
            break; // 接收到特殊值 0 表示所有数据都已接收完毕，停止接收
        }
        println!("Got: {}", received);
        total_received += 1;
    }

    println!("total numbers received: {}", total_received);
    assert_eq!(total_received, queue_length)
}