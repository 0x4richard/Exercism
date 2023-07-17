use std::collections::HashMap;

/// Count occurrences of words.
pub fn word_count(words: &str) -> HashMap<String, u32> {
    words
        .chars()
        .map(|c| {
            if c.is_ascii_punctuation() && c != '\'' {
                ' '
            } else {
                c.to_ascii_lowercase()
            }
        })
        .collect::<String>()
        .split_ascii_whitespace()
        .fold(HashMap::new(), |mut counter, word| {
            let c = counter
                .entry(String::from(word.trim_matches('\'')))
                .or_insert(0);
            *c += 1;
            counter
        })
}
