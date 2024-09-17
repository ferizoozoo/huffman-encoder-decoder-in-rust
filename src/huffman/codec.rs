use std::{
    fs::File,
    io::{BufRead, BufReader, Read, Write},
};

use super::prefix_code::{PrefixCodeTable, TableMethods};

const BUFFER_SIZE: usize = 1024;

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
    fn parse_header_into_prefix_code_table(
        input_filename: String,
    ) -> Result<PrefixCodeTable, std::io::Error>;
    fn decode(
        prefix_code_table: PrefixCodeTable,
        input_filename: String,
        output_filename: String,
    ) -> std::io::Result<()>;
}

pub struct Codec {
    input_filename: String,
    output_filename: String,
    prefix_code_table: PrefixCodeTable,
}

impl Codec {
    fn get_header(prefix_code_table: PrefixCodeTable) -> Vec<u8> {
        return [prefix_code_table.stringify().as_bytes(), b"\n"].concat();
    }
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

        let header = Codec::get_header(prefix_code_table.clone());
        output.write_all(&header)?;

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
    fn parse_header_into_prefix_code_table(
        input_filename: String,
    ) -> Result<PrefixCodeTable, std::io::Error> {
        let input = File::open(input_filename)?;

        let mut reader = BufReader::new(input);

        let mut buf = Vec::<u8>::new();
        reader.read_until(b'\n', &mut buf)?;

        let prefix_table_string = String::from_utf8(buf)
            .expect("Could not convert prefix code table from bytes to string");
        return Ok(PrefixCodeTable::to_table(prefix_table_string));
    }

    fn decode(
        prefix_code_table: PrefixCodeTable,
        input_filename: String,
        output_filename: String,
    ) -> std::io::Result<()> {
        todo!()
    }
}
