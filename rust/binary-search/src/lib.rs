use std::cmp::Ordering;

pub fn find(array: &[i32], key: i32) -> Option<usize> {
    return binary_search(array, key, 0);
}

fn binary_search(array: &[i32], key: i32, index: usize) -> Option<usize> {
    let len = array.len();
    if len <= 0 {
        return None;
    }

    let mid = len / 2;
    let current = array[mid];
    match key.cmp(&current) {
        Ordering::Equal => return Some(mid + index),
        Ordering::Less => return binary_search(&array[..mid], key, index),
        Ordering::Greater => return binary_search(&array[mid + 1..], key, mid + 1 + index),
    }
}
