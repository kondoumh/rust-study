use std::thread;
use std::time::Duration;

fn main() {
    let handle = thread::spawn(move || {
        for i in 1..10 {
            println!("hi number {} from spawned thread!", i);
            thread::sleep(Duration::from_millis(1));
        }
    });
    
    //handle.join().unwrap();

    for i in 1..5 {
        println!("hi number {} from main thread!", i);
        thread::sleep(Duration::from_millis(1));
    }

    handle.join().unwrap();
}
