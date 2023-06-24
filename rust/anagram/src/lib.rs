use std::collections::HashSet;

pub fn anagrams_for<'a>(word: &str, possible_anagrams: &'a [&'a str]) -> HashSet<&'a str> {
    let sorted_workd = sort_word(word);

    let mut set = HashSet::new();

    for w in possible_anagrams {
        let sw = sort_word(w);
        if sorted_workd == sw && to_lowercase(word) != to_lowercase(w) {
            set.insert(w.as_ref());
        }
    }

    set
}

fn to_lowercase_fn(c: char) -> String {
    if c.is_lowercase() {
        c.to_string()
    } else {
        c.to_lowercase().to_string()
    }
}

fn to_lowercase(word: &str) -> String {
    word.chars().map(to_lowercase_fn).collect::<String>()
}

fn sort_word(word: &str) -> String {
    let mut sorted_word = to_lowercase(word).chars().collect::<Vec<char>>();
    sorted_word.sort_unstable();
    sorted_word.into_iter().collect::<String>()
}
