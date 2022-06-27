use std::collections::HashMap;
use std::sync::{Arc, Mutex};
use std::thread;

pub fn frequency(input: &'static &[&str], worker_count: usize) -> HashMap<char, usize> {
    let counter: Arc<Mutex<HashMap<char, usize>>> = Arc::new(Mutex::new(HashMap::new()));
    let len: Arc<usize> = Arc::new(input.len());
    let data = Arc::new(input);
    let mut handles = vec![];

    for i in 0..worker_count
    {
        let counter = Arc::clone(&counter);
        let len = Arc::clone(&len);
        let data = Arc::clone(&data);
        let strings_to_handle = match input.len() % worker_count
        {
            0 => input.len() / worker_count,
            _ => input.len() / worker_count + 1
        };
        let handle = thread::spawn(move ||
            {
                for j in strings_to_handle * i .. (strings_to_handle * (i + 1))
                {
                    match j
                    {
                        j if j < *len =>
                            {
                                let _ = (*data).get(j)
                                    .unwrap()
                                    .chars()
                                    .into_iter()
                                    .map(|_char| {
                                        let mut counter = counter.lock().unwrap();
                                        match counter.get(&_char)
                                        {
                                            Some(amount) => counter.insert(_char, amount + 1),
                                            None => counter.insert(_char, 1)
                                        }
                                    });
                            },
                        _ => {}
                    }
                }
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
