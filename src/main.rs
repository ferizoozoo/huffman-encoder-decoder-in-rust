use std::{collections::HashMap, env, fs};

use huffman_tree::HuffmanTree;
use prefix_code::{PrefixCodeTable, TableMethods};

mod huffman_node;
mod huffman_tree;
mod prefix_code;

fn filename_arg_parser(args: Vec<String>) -> Result<String, &'static str> {
    if args.len() != 2 {
        return Err("Filename is not provided");
    }

    let filename = args[1].clone();
    return Ok(filename);
}

fn get_word_frequency(contents: String) -> HashMap<char, u32> {
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

    return freq;
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let filename = filename_arg_parser(args).unwrap();
    let contents = fs::read_to_string(filename).expect("Could not read the file");
    let freq_table = get_word_frequency(contents);
    let tree = HuffmanTree::new(freq_table);
    let prefix_code_table = PrefixCodeTable::create(tree);
    dbg!(prefix_code_table);
}
