use std::collections::HashSet;

pub fn anagrams_for<'a>(word: &str, possible_anagrams: &'a [&str]) -> HashSet<&'a str> {
    let lower_case_word = word.to_lowercase();
    let lower_case_sorted_word = lower_case_and_sort(word);
    let mut anagrams = HashSet::new();

    possible_anagrams
        .iter()
        .filter(|&&candidate| {
            let lower_case_candidate = candidate.to_lowercase();
            candidate.len() == word.len()
                && lower_case_candidate != lower_case_word
                && lower_case_sorted_word == lower_case_and_sort(candidate)
        })
        .for_each(|&candidate| {
            anagrams.insert(candidate);
        });

    anagrams
}

fn lower_case_and_sort(word: &str) -> String {
    let mut chars: Vec<char> = word.to_lowercase().chars().collect();
    chars.sort_unstable();
    chars.into_iter().collect()
}
