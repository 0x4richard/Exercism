use std::{collections::HashMap, str, thread};

pub fn frequency(input: &[&str], worker_count: usize) -> HashMap<char, usize> {
    let mut map = HashMap::new();
    let input_str = input.join("");
    if input_str.is_empty() {
        return map;
    }
    let chunk_size = (input_str.len() as f32 / worker_count as f32).ceil() as usize;
    let mut handles = Vec::with_capacity(chunk_size);

    input_str.as_bytes().chunks(chunk_size).for_each(|chunk| {
        let s = str::from_utf8(chunk)
            .expect("Found invalid UTF-8")
            .to_lowercase();

        let handle = thread::spawn(move || {
            let mut hm = HashMap::<char, usize>::new();
            s.chars().for_each(|c| {
                if c.is_ascii_punctuation() || c.is_numeric() {
                    return;
                }

                match hm.get_mut(&c) {
                    Some(count) => *count += 1,
                    None => {
                        hm.insert(c, 1);
                        ()
                    }
                }
            });

            hm
        });
        handles.push(handle);
    });

    for handle in handles {
        handle
            .join()
            .unwrap()
            .into_iter()
            .for_each(|(k, v)| *map.entry(k).or_default() += v);
    }

    map
}
