/*
In this step your goal is to write a header section to the output file
(you’ll want a command line option to specify the filename).
The header section will include enough information for your program to be decode the compressed file.

One way of doing this is to write out the tree,
another option is to write out the character frequency table.
Don’t forget you’ll need some way of knowing when the header ends and when the compressed data starts.
 */

use std::{collections::HashMap, env, fs};

mod huffman;

use huffman::prefix_code::TableMethods;

pub use crate::huffman::huffman_tree::HuffmanTree;
pub use crate::huffman::prefix_code::PrefixCodeTable;

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

fn write_header_to_output(filename: String, freq_table: PrefixCodeTable) {
    //TODO: get string representation of prefix table and write it as
    //      a header in the output file
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
