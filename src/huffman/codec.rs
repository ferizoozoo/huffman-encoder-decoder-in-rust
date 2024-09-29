use std::{
    fs::File,
    io::{BufRead, BufReader, BufWriter, Read, Write},
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
    fn decode(input_filename: String, output_filename: String) -> std::io::Result<()>;
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
        prefix_code_table: PrefixCodeTable, // TODO: should receive reference instead of value
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
    fn decode(input_filename: String, output_filename: String) -> std::io::Result<()> {
        let input = File::open(input_filename)?;
        let output = File::create(output_filename)?;
        let mut reader = BufReader::new(input);
        let mut writer = BufWriter::new(output);

        // Parse header of encoded file (prefix code table)
        let mut buf = Vec::<u8>::new();
        reader.read_until(b'\n', &mut buf)?;

        let prefix_table_string = String::from_utf8(buf)
            .expect("Could not convert prefix code table from bytes to string");

        let prefix_code_table = PrefixCodeTable::to_table(prefix_table_string);
        let reversed_table = prefix_code_table.reverse();

        // Parse body of encoded file
        let mut key = String::new();

        loop {
            let mut buf: [u8; BUFFER_SIZE] = [0; BUFFER_SIZE];
            let limit = reader.read(buf.as_mut_slice())?;

            if limit == 0 {
                break;
            }

            for byte in buf.iter().take(limit) {
                // Process each bit of the byte
                for bit_pos in 0..8 {
                    let bit = (byte >> (7 - bit_pos)) & 1;
                    key.push(if bit == 1 { '1' } else { '0' });

                    if let Some(ch) = reversed_table.get(&key) {
                        // Key matched, write the decoded character
                        writer.write_all(&[*ch as u8])?;
                        writer.flush()?;
                        key.clear();
                    }
                }
            }
        }

        Ok(())
    }
}
