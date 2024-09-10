use std::{
    fs::File,
    io::{Read, Write},
};

use super::prefix_code::PrefixCodeTable;

const BUFFER_SIZE: usize = 1024;

pub enum CodecType {
    Encoding,
    Decoding,
}

pub trait Encoder {
    //TODO: use Codec struct instead of these parameters
    fn encode(
        prefix_code_table: PrefixCodeTable,
        input_filename: String,
        output_filename: String,
    ) -> std::io::Result<()>;
}

pub trait Decoder {
    //TODO: use Codec struct instead of these parameters
    fn decode(
        prefix_code_table: PrefixCodeTable,
        input_filename: String,
        output_filename: String,
    ) -> std::io::Result<()>;
}

pub struct Codec {
    input_filename: String,
    output_filename: String,
    codec_type: CodecType,
    prefix_code_table: PrefixCodeTable,
}

impl Encoder for Codec {
    //TODO: use self instead of these parameters
    fn encode(
        prefix_code_table: PrefixCodeTable,
        input_filename: String,
        output_filename: String,
    ) -> std::io::Result<()> {
        let mut input = File::open(input_filename)?;
        let mut output = File::create(output_filename)?;
        let mut buffer: [u8; BUFFER_SIZE] = [0; BUFFER_SIZE];

        loop {
            let limit = input.read(buffer.as_mut_slice())?;

            if limit == 0 {
                break;
            }

            let mut encoded_data = String::new();

            buffer.iter().for_each(|b| {
                let character = *b as char;
                if let Some(encoded_char) = prefix_code_table.get(&character) {
                    encoded_data.push_str(encoded_char);
                }
            });

            let encoded_data_bytes = encoded_data
                .as_bytes()
                .chunks(8)
                .map(|chunk| {
                    let chunk_str = std::str::from_utf8(chunk).unwrap();
                    u8::from_str_radix(chunk_str, 2).unwrap()
                })
                .collect::<Vec<u8>>();
            output.write_all(&encoded_data_bytes)?;
        }

        return Ok(());
    }
}

impl Decoder for Codec {
    //TODO: use self instead of these parameters
    fn decode(
        prefix_code_table: PrefixCodeTable,
        input_filename: String,
        output_filename: String,
    ) -> std::io::Result<()> {
        todo!()
    }
}
