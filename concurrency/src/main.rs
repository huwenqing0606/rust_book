use std::{thread, time};

fn main() {

    let ten_millis = time::Duration::from_millis(10);

    let handle = thread::spawn(|| {
        for i in 1..20 {
            println!("hi number {} from the spawn thread", i);
        }
    });

    for i in 1..10 {
        println!("hi number {} from the main thread!", i);
        thread::sleep(ten_millis);
    }

    handle.join();
}