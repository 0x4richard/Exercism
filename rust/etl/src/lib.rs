use std::collections::BTreeMap;

pub fn transform(h: &BTreeMap<i32, Vec<char>>) -> BTreeMap<char, i32> {
    let mut result = BTreeMap::new();
    for (&k, v) in h {
        for &el in v {
            let e = el.to_ascii_lowercase();
            result.insert(e, k);
        }
    }

    result
}
