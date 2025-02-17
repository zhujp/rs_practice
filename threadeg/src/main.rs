use std::sync::mpsc;
use std::thread;
// use std::time::Duration;
fn main() {
    // thread::spawn(|| {
    //     for i in 1..10 {
    //         println!("hello world {}", i);
    //         thread::sleep(Duration::from_millis(1));
    //     }
    // });

    // for i in 1..5 {
    //     println!("main thread {}", i);
    //     thread::sleep(Duration::from_millis(1));

    // } //主线程结束，其它线程也结束

    let (tx, rx) = mpsc::channel();

    thread::spawn(move || {
        let val = String::from("hi");
        println!("val is {}", val);
        tx.send(val).unwrap();
    });

    let received = rx.recv().unwrap();
    println!("Got: {}", received);
}
