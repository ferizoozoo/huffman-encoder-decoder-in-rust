use std::{
    collections::HashMap,
    fs::File,
    io::{BufRead, BufReader, BufWriter, Read, Write},
};

use super::prefix_code::{PrefixCodeTable, TableMethods};

const BUFFER_SIZE: usize = 1024;
const HEADER_END_BYTE: u8 = b'\0';

pub struct Codec;

impl Codec {
    fn get_header(prefix_code_table: PrefixCodeTable) -> Vec<u8> {
        return [
            prefix_code_table.stringify().as_bytes(),
            &HEADER_END_BYTE.to_le_bytes(),
        ]
        .concat();
    }

    pub fn encode(
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

    pub fn decode(
        input_filename: String,
        output_filename: String,
        max_bits: usize,
    ) -> std::io::Result<()> {
        let input = File::open(input_filename)?;
        let output = File::create(output_filename)?;
        let mut reader = BufReader::new(input);
        let mut writer = BufWriter::new(output);

        let reversed_table = Codec::decode_header(&mut reader)?;
        Codec::decode_body(&mut reader, &mut writer, reversed_table, max_bits)
    }

    fn decode_header(reader: &mut BufReader<File>) -> std::io::Result<HashMap<String, char>> {
        let mut buf = Vec::<u8>::new();
        reader.read_until(HEADER_END_BYTE, &mut buf)?;

        if let Some(pos) = buf.iter().position(|&x| x == HEADER_END_BYTE) {
            buf.truncate(pos);
        }

        let prefix_table_string = String::from_utf8(buf)
            .expect("Could not convert prefix code table from bytes to string");

        let prefix_code_table = PrefixCodeTable::to_table(prefix_table_string);
        return Ok(prefix_code_table.reverse());
    }

    fn decode_body(
        reader: &mut BufReader<File>,
        writer: &mut BufWriter<File>,
        reversed_table: HashMap<String, char>,
        max_bits: usize,
    ) -> std::io::Result<()> {
        let mut key = String::new();

        let mut buffer = [0; BUFFER_SIZE];
        loop {
            let limit = reader.read(&mut buffer)?;
            if limit == 0 {
                break;
            }

            for byte in buffer.iter().take(limit) {
                for bit_pos in 0..8 {
                    let bit = (byte >> (7 - bit_pos)) & 1;
                    key.push(if bit == 1 { '1' } else { '0' });

                    if let Some(&ch) = reversed_table.get(&key) {
                        // encode ch as a UTF-8 character
                        let mut buf = [0; 4];
                        let ch_utf8 = ch.encode_utf8(&mut buf);
                        writer.write_all(ch_utf8.as_bytes())?;
                        key.clear();
                    }

                    if key.len() > max_bits {
                        key.clear();
                    }
                }
            }
        }

        writer.flush()?;
        Ok(())
    }
}
