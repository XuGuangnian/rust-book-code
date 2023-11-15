use std::rc::Rc;
use std::sync::{Arc, Mutex};
use std::thread;

pub fn mutex() {
    println!("==================== mutex ====================");
    let m = Mutex::new(5);

    {
        let mut num = m.lock().unwrap(); // 这个调用会阻塞当前线程，直到我们拥有锁为止。
        *num = 6;
    }

    println!("m = {:?}", m);
}

pub fn shared_mutex() {
    println!("==================== shared_mutex ====================");
    let counter = Arc::new(Mutex::new(0));
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
