use std::collections::HashSet;

pub fn anagrams_for<'a>(word: &str, possible_anagrams: &'a [&'a str]) -> HashSet<&'a str> {
    let sorted_workd = sort_word(word);

    let mut set = HashSet::new();

    for w in possible_anagrams {
        let sw = sort_word(w);
        if sorted_workd == sw && word.to_string().to_lowercase() != w.to_string().to_lowercase() {
            set.insert(w.as_ref());
        }
    }

    set
}

fn sort_word(word: &str) -> String {
    let mut sorted_word = word.to_lowercase().chars().collect::<Vec<char>>();
    sorted_word.sort_unstable();
    sorted_word.into_iter().collect::<String>()
}
