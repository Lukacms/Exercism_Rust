use std::collections::HashSet;

fn sort(word: &str) -> Vec<char> {
    let mut sorted: Vec<char> = word.chars().collect();

    sorted.sort_unstable();
    sorted
}

pub fn anagrams_for<'a>(word: &str, possible_anagrams: &[&'a str]) -> HashSet<&'a str> {
    let lower = word.to_lowercase();
    let sorted: Vec<char> = sort(&lower);

    possible_anagrams
        .iter()
        .filter(|candidate| {
            let candidate_lower = candidate.to_lowercase();
            candidate_lower.len() == lower.len()
                && candidate_lower != lower
                && sort(&candidate_lower) == sorted
        })
        .copied()
        .collect()
}
