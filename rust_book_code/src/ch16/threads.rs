use std::thread;
use std::time::Duration;

pub fn create() {
    let handle = thread::spawn(|| {
        for i in 0..10 {
            println!("hi number {} from the spawned thread!", i);
            thread::sleep(Duration::from_millis(1));
        }
    });

    handle.join().unwrap(); // join会等待线程执行完毕

    for i in 0..5 {
        println!("hi number {} from the main thread!", i);
        thread::sleep(Duration::from_millis(1));
    }
}

pub fn move_closure() {
    let v = vec![1, 2, 3];

    let handle = thread::spawn(move || {
        println!("Here's a vector: {:?}", v);
    });

    // drop(v);

    handle.join().unwrap();
}
