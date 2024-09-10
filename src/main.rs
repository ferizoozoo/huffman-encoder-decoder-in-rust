use std::fs::File;
use std::io::Write;
use std::{collections::HashMap, env, fs};

mod huffman;

use huffman::codec::{Codec, Encoder};
use huffman::prefix_code::TableMethods;

pub use crate::huffman::huffman_tree::HuffmanTree;
pub use crate::huffman::prefix_code::PrefixCodeTable;

fn filename_arg_parser(args: Vec<String>) -> Result<String, &'static str> {
    if args.len() < 2 {
        return Err("Filename is not provided");
    }

    let filename = args[1].clone();
    return Ok(filename);
}

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

fn get_word_frequency(contents: String) -> HashMap<char, u32> {
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

//TODO: I think this function should be placed in codec.rs
fn write_encoded_file_to_output(
    filename: String,
    prefix_code_table: PrefixCodeTable,
) -> std::io::Result<()> {
    // writing header to the output file
    let mut file = File::create(filename)?;

    let header = [b"begin ", prefix_code_table.stringify().as_bytes(), b" end"].concat();

    file.write_all(&header)?;

    //TODO: write encoded file as chunks into the output file

    return Ok(());
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let input_filename = filename_arg_parser(args.clone()).unwrap();
    let output_filename = option_arg_parser(args).unwrap();
    let contents = fs::read_to_string(&input_filename).expect("Could not read the file");
    let freq_table = get_word_frequency(contents);
    let tree = HuffmanTree::new(freq_table);
    let prefix_code_table = PrefixCodeTable::create(tree);
    let msg =
        write_encoded_file_to_output(output_filename.clone(), prefix_code_table.clone()).unwrap();
    match Codec::encode(prefix_code_table, input_filename, output_filename) {
        Ok(r) => (),
        Err(e) => panic!("{}", e),
    }
}
