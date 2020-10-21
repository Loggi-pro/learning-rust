use std::sync::mpsc;
use std::thread;
use std::time::Duration;
#[allow(dead_code)]
pub fn run() {
    println!("Create new 'multiple producer, single consumer' channel");
    let (tx, rx) = mpsc::channel(); //first element is tranceiver end, second->is receiving end
    let tx1 = mpsc::Sender::clone(&tx); //create transmitter for thread
    thread::spawn(move || {
        let vals = vec![
            String::from("hi"),
            String::from("from"),
            String::from("the"),
            String::from("thread"),
        ];
        for val in vals {
            tx1.send(val).unwrap();
            thread::sleep(Duration::from_millis(300));
        }
        //cant use 'val' here anymore (it moved away)
    });
    //make second transmitter
    thread::spawn(move || {
        let vals = vec![
            String::from("more"),
            String::from("messages"),
            String::from("to"),
            String::from("you"),
        ];
        for val in vals {
            tx.send(val).unwrap();
            thread::sleep(Duration::from_millis(300));
        }
        //cant use 'val' here anymore (it moved away)
    });

    //get single value
    let received = rx.recv().unwrap();
    println!("Got blocked single: '{}' from thread", received);
    //get by iterator
    for received in rx {
        println!("Got blocked iterated: '{}' from thread", received);
    }
    println!("For non blocking receiving use 'try_recv'");
    //
}
