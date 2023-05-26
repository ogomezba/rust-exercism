use std::collections::HashSet;

pub fn anagrams_for<'a>(word: &str, possible_anagrams: &'a [&str]) -> HashSet<&'a str> {
    let lower_case_word = word.clone().to_lowercase();

    let sorted_word = clone_and_sort_and_lower(word).to_lowercase();
    let mut anagrams = HashSet::new();

    possible_anagrams
        .iter()
        .filter(|&candidate| {
            if word.len() != candidate.len() {
                false
            } else {
                let lower_case_candidate = candidate.clone().to_lowercase();
                let sorted_candidate = clone_and_sort_and_lower(candidate);

                if lower_case_candidate == lower_case_word {
                    false
                } else {
                    sorted_candidate == sorted_word
                }
            }
        })
        .for_each(|&candidate| {
            anagrams.insert(candidate);
        });

    anagrams
}

pub fn clone_and_sort_and_lower(word: &str) -> String {
    let char_array: Vec<char> = word.chars().collect();

    let mut string_array: Vec<String> = char_array
        .iter()
        .map(|char| char.to_lowercase().to_string())
        .collect();

    string_array.sort_unstable();

    string_array.join("")
}
