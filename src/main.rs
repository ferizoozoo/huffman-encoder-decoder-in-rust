use std::{collections::HashMap, env, fs};

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() != 2 {
        panic!("Filename is not provided");
    }

    let filename = args[1].clone();

    let contents = fs::read_to_string(filename).expect("Could not read the file");
    let mut freq: HashMap<char, u32> = HashMap::new();

    for c in contents.chars() {
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
