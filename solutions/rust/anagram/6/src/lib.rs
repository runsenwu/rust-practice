use std::collections::{HashMap, HashSet};

pub fn anagrams_for<'a>(word: &str, possible_anagrams: &[&'a str]) -> HashSet<&'a str> {
    // first, how would I approach the basic needs here?

    let mut ret: HashSet<&'a str> = HashSet::new();
    let lower_case_word = word.to_lowercase();
    let mut diction: HashMap<char, i32> = HashMap::new();
    let mut compare_to: HashMap<char, i32> = HashMap::new();

    for c in lower_case_word.chars() {
        let value = diction.entry(c).or_insert(0);
        *value += 1;
    }

    for &possible in possible_anagrams {
        let lower_possible = possible.to_lowercase();
        
        if lower_case_word == lower_possible {
            continue;
        }

        for c in lower_possible.chars() {
            let value = compare_to.entry(c).or_insert(0);
            *value += 1
        }

        if diction == compare_to {
            ret.insert(possible);
        }
    }

    ret

    // need a dashmap
    // let mut ret: HashSet<&'a str> = HashSet::new();
    // let lower_case_word = word.to_lowercase();
    // let mut sorted_word: Vec<char> = lower_case_word.to_lowercase().chars().collect();
    // sorted_word.sort_unstable();

    // for &possible_anagram in possible_anagrams {
    //     let lower_case_temp = possible_anagram.to_lowercase();

    //     if lower_case_word == lower_case_temp {
    //         continue;
    //     }
        
    //     let mut temp_sorted_word: Vec<char> = lower_case_temp.chars().collect::<Vec<char>>();
    //     temp_sorted_word.sort_unstable();

    //     if temp_sorted_word == sorted_word {
    //         ret.insert(possible_anagram);
    //     }
    // }

    // ret
}
