use std::collections::HashMap;

pub fn get_word_frequency(contents: String) -> HashMap<char, u32> {
    let mut freq: HashMap<char, u32> = HashMap::new();

    for c in contents.chars() {
        if c.is_alphanumeric() {
            match freq.get(&c) {
                Some(count) => {
                    freq.insert(c, count + 1);
                }
                None => {
                    freq.insert(c, 1);
                }
            }
        }
    }

    return freq;
}
