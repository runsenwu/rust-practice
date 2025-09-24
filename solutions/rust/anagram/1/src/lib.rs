use std::collections::HashSet;

pub fn anagrams_for<'a>(word: &str, possible_anagrams: &[&'a str]) -> HashSet<&'a str> {
    // first, how would I approach the basic needs here?

    // need a dashmap
    let mut ret: HashSet<&str> = HashSet::new();
    let lower_case_word = word.to_lowercase();
    let mut sorted_word: Vec<char> = word.to_lowercase().chars().collect();
    sorted_word.sort_unstable();

    for possible_anagram in possible_anagrams {
        let lower_case_temp = word.to_lowercase();

        if lower_case_word == lower_case_temp {
            continue;
        }
        
        let mut temp_sorted_word: Vec<char> = (*possible_anagram).to_lowercase().chars().collect::<Vec<char>>();
        temp_sorted_word.sort_unstable();

        if temp_sorted_word == sorted_word {
            ret.insert(possible_anagram);
        }
    }

    ret
}
