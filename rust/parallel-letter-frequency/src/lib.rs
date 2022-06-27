use std::collections::HashMap;
use std::sync::{Arc, Mutex};
use std::thread;
use std::thread::JoinHandle;

pub fn frequency(input: &[&str], worker_count: usize) -> HashMap<char, usize> {
    let counter: Arc<Mutex<HashMap<char, usize>>> = Arc::new(Mutex::new(HashMap::new()));
    let mut handles = vec![];

    for i in 0..worker_count
    {
        let counter = Arc::clone(&counter);
        let handle = thread::spawn(||
            {

            });
        handles.push(handle);
    }

    for handle in handles
    {
        handle.join().unwrap();
    }

    let map = counter.lock().unwrap().to_owned();
    map
}
