use std::sync::{mpsc, Arc, Mutex};
use std::thread;
use std::time::Duration;
use rand::Rng;

// Define a special value that will signal termination
const TERMINATION_SIGNAL: i32 = -1;

fn main() {
    // Number of items to produce
    const ITEM_COUNT: usize = 20;
    
    // TODO: Create a channel for sending numbers
    let (tx, rx) = mpsc::channel();
    let rx = Arc::new(Mutex::new(rx));

    let mut producer_handles = vec![];
    let mut consumer_handles = vec![];
    
    // TODO: Create 2 producer threads
    for i in 0..2 {
        let tx_clone = tx.clone();
        let handle = thread::spawn(move || {
            producer(i, tx_clone, ITEM_COUNT / 2);
        });
        producer_handles.push(handle);
    }
    
    // TODO: Create 3 consumer threads
    for i in 0..3 {
        let rx_clone = Arc::clone(&rx);
        let handle = thread::spawn(move || {
            consumer(i, rx_clone);
        });
        consumer_handles.push(handle);
    }
    
    // TODO: Wait for all threads to finish
    for handle in producer_handles {
        handle.join().unwrap();
    }

    println!("All producers finished. Sending termination signals.");

    for _ in 0..3 {
        tx.send(TERMINATION_SIGNAL).unwrap();
    }
    
    for handle in consumer_handles {
        handle.join().unwrap();
    }
    
    println!("All items have been produced and consumed!");
}


// TODO: Implement producer function
fn producer(id: usize, tx: mpsc::Sender<i32>, item_count: usize) {
    // TODO: Generate random numbers and send them to the channel
    // When finished, producer should NOT send termination signal

    let mut rng = rand::thread_rng();

    for _ in 0..item_count {
        let num = rng.gen_range(1..100);
        println!("Producer {} produced: {}", id, num);
        tx.send(num).unwrap();
        thread::sleep(Duration::from_millis(200));
    }
    println!("Producer {} finished", id);
}

// TODO: Implement consumer function
fn consumer(id: usize, rx: Arc<Mutex<mpsc::Receiver<i32>>>) {
    // TODO: Receive numbers from the channel and process them
    // Break the loop when receiving the termination signal

    loop {
        let value = rx.lock().unwrap().recv().unwrap();
        
        if value == TERMINATION_SIGNAL {
            println!("Consumer {} received termination signal.", id);
            break;
    }

    println!("Consumer {} consumed: {}", id, value);
    
    thread::sleep(Duration::from_millis(300));
    }
    println!("Consumer {} finished", id);

}