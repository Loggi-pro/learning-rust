use std::sync::{Arc, Mutex};
use std::thread;
#[allow(dead_code)]
pub fn run() {
    println!("We cannot share just mutex. We need to make copy of this, multiple owners ->Rc");
    let counter = Arc::new(Mutex::new(0)); //Arc is atomic version of Rc
                                           //counter is immutable but we can mutate it like RefCell
    let mut handles = vec![];
    for _ in 0..10 {
        let counter = Arc::clone(&counter);
        let handle = thread::spawn(move || {
            let mut num = counter.lock().unwrap();
            *num += 1;
        });
        handles.push(handle);
    }

    for handle in handles {
        handle.join().unwrap();
    }

    println!("Result: {}", *counter.lock().unwrap());
}
