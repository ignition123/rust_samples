use std::sync::{Arc, Mutex};
use std::thread;

fn main()
{
    let mtx = Arc::new(Mutex::new(""));

    let mtx1 = mtx.clone();

    let mtx2 = mtx.clone();

    let n = 50;

    let th1 = thread::spawn(move || {

        mtx1.lock().unwrap();

        printData(n, "*".to_string());

    });

    let th2 = thread::spawn(move || {

        mtx2.lock().unwrap();

        printData(n, "$".to_string());

    });

    th1.join();
    th2.join();
}

fn printData(n:u32, c:String)
{
    let mut str_val:String = "".to_string();

    for i in 0..n
    {
        str_val.push_str(&c);
    }

    println!("{}", str_val);
}