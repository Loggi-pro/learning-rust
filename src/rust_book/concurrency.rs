use std::thread;
use std::time::Duration;

#[allow(dead_code)]
pub fn run() {
    println!("Spawn threads");
    let v: Vec<u32> = (1..10).collect();
    let handle = thread::spawn(move || {
        for i in v {
            println!("Hi number {} from the spawned thread!", i);
        }
    });
    for i in 1..5 {
        println!("Hi number {} from the main thread!", i);
        thread::sleep(Duration::from_millis(1));
    }
    println!("Wait thread to complete!");
    handle.join().unwrap();
}
