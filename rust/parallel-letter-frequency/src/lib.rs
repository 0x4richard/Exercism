use std::{
    collections::HashMap,
    str,
    sync::{Arc, Mutex},
    thread,
};

pub fn frequency(input: &[&str], worker_count: usize) -> HashMap<char, usize> {
    let hm_arc: Arc<Mutex<HashMap<char, usize>>> = Arc::new(Mutex::new(HashMap::new()));

    let input_str = input.join("");
    if input_str.is_empty() {
        return HashMap::new();
    }
    let chunk_size = (input_str.len() as f32 / worker_count as f32).ceil() as usize;
    let mut handles = Vec::with_capacity(chunk_size);

    input_str.as_bytes().chunks(chunk_size).for_each(|chunk| {
        let s = str::from_utf8(chunk)
            .expect("Found invalid UTF-8")
            .to_lowercase();

        let hm_arc = Arc::clone(&hm_arc);
        let handle = thread::spawn(move || {
            s.chars().for_each(|c| {
                println!("{}", c);

                if c.is_ascii_punctuation() || c.is_numeric() {
                    return;
                }

                let mut hm = hm_arc.lock().unwrap();
                match hm.get_mut(&c) {
                    Some(count) => *count += 1,
                    None => {
                        hm.insert(c, 1);
                        ()
                    }
                }
            });
        });
        handles.push(handle);
    });

    for handle in handles {
        handle.join().unwrap();
    }

    Arc::try_unwrap(hm_arc).unwrap().into_inner().unwrap()
}
