use std::{env, fs};

mod huffman;
mod utils;

use huffman::codec::Codec;
use huffman::prefix_code::TableMethods;
use huffman::word_frequency::get_word_frequency;
use utils::arg_parser::{filename_arg_parser, option_arg_parser};

pub use crate::huffman::huffman_tree::HuffmanTree;
pub use crate::huffman::prefix_code::PrefixCodeTable;

fn main() {
    let args: Vec<String> = env::args().collect();
    let input_filename = filename_arg_parser(args.clone()).unwrap();
    let output_filename = option_arg_parser(args).unwrap();
    let decode_filename = String::from("decoded_output.dec");
    let contents = fs::read_to_string(&input_filename).expect("Could not read the file");
    let freq_table = get_word_frequency(contents);
    let tree = HuffmanTree::new(freq_table);
    let prefix_code_table = PrefixCodeTable::create(&tree);
    match Codec::encode(
        &prefix_code_table,
        input_filename.clone(),
        output_filename.clone(),
    ) {
        Ok(_r) => (),
        Err(e) => panic!("{}", e),
    }
    let padding_byte_count = Codec::get_padding_byte_count(&input_filename).unwrap();
    match Codec::decode(output_filename.clone(), decode_filename, padding_byte_count) {
        Ok(_r) => (),
        Err(e) => panic!("{}", e),
    }
}
