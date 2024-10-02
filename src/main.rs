use std::{env, fs};

mod huffman;

use huffman::codec::Codec;
use huffman::prefix_code::TableMethods;
use huffman::word_frequency::get_word_frequency;

pub use crate::huffman::huffman_tree::HuffmanTree;
pub use crate::huffman::prefix_code::PrefixCodeTable;

// TODO: this should be placed into a utility module or something
fn filename_arg_parser(args: Vec<String>) -> Result<String, &'static str> {
    if args.len() < 2 {
        return Err("Filename is not provided");
    }

    let filename = args[1].clone();
    return Ok(filename);
}

// TODO: this should be placed into a utility module or something
fn option_arg_parser(args: Vec<String>) -> Result<String, &'static str> {
    const OUTPUT_FILE_OPTION: &str = "-o";

    if args.len() < 4 {
        return Err("Output option and filename are not provided");
    }

    let index = args
        .iter()
        .position(|arg| arg == OUTPUT_FILE_OPTION)
        .unwrap();
    let filename = args[index + 1].clone();
    return Ok(filename);
}

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
        prefix_code_table.clone(),
        input_filename.clone(),
        output_filename.clone(),
    ) {
        Ok(_r) => (),
        Err(e) => panic!("{}", e),
    }
    match Codec::decode(output_filename.clone(), decode_filename, tree.depth()) {
        Ok(_r) => (),
        Err(e) => panic!("{}", e),
    }
}
