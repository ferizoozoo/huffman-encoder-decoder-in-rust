use std::collections::HashMap;

pub fn get_word_frequency(contents: String) -> HashMap<char, u32> {
    let mut freq: HashMap<char, u32> = HashMap::new();

    for c in contents.chars() {
        *freq.entry(c).or_insert(1) += 1;
    }

    return freq;
}
