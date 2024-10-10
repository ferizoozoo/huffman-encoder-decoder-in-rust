use std::{
    collections::HashMap,
    fs::File,
    io::{BufRead, BufReader, BufWriter, Read, Write},
};

use super::prefix_code::{PrefixCodeTable, TableMethods};

const BUFFER_SIZE: usize = 8192;
const HEADER_END_BYTE: u8 = b'\0';

pub struct Codec;

impl Codec {
    fn create_header_from_prefix_table(prefix_code_table: &PrefixCodeTable) -> Vec<u8> {
        return [
            prefix_code_table.stringify().as_bytes(),
            &HEADER_END_BYTE.to_le_bytes(),
        ]
        .concat();
    }

    pub fn get_padding_byte_count(filename: &String) -> std::io::Result<usize> {
        let reader = File::open(filename)?;
        return Ok((reader.metadata()?.len() % 8) as usize);
    }

    pub fn encode(
        prefix_code_table: &PrefixCodeTable, // TODO: should receive reference instead of value
        input_filename: String,
        output_filename: String,
    ) -> std::io::Result<()> {
        let mut input = File::open(input_filename)?;
        let mut output = File::create(output_filename)?;
        let mut buffer: [u8; BUFFER_SIZE] = [0; BUFFER_SIZE];
        let mut leftover = Vec::new();

        let header = Codec::create_header_from_prefix_table(&prefix_code_table);
        output.write_all(&header)?;

        loop {
            let limit = input.read(buffer.as_mut_slice())?;

            if limit == 0 {
                break;
            }

            leftover.extend_from_slice(&buffer[..limit]);

            let valid_up_to = match std::str::from_utf8(&leftover) {
                Ok(valid_str) => {
                    Codec::process_valid_utf8(&valid_str, &prefix_code_table, &mut output)?;
                    leftover.clear();
                    leftover.len()
                }
                Err(e) => {
                    let valid_str = &leftover[..e.valid_up_to()];
                    Codec::process_valid_utf8(
                        std::str::from_utf8(valid_str).unwrap(),
                        &prefix_code_table,
                        &mut output,
                    )?;
                    e.valid_up_to()
                }
            };

            let remainder = leftover.split_off(valid_up_to);
            leftover.clear();
            leftover.extend_from_slice(&remainder);
        }

        return Ok(());
    }

    fn process_valid_utf8(
        valid_str: &str,
        prefix_code_table: &PrefixCodeTable,
        output: &mut File,
    ) -> std::io::Result<()> {
        let mut encoded_data = String::new();

        valid_str.chars().for_each(|character| {
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

        Ok(())
    }

    pub fn decode(
        input_filename: String,
        output_filename: String,
        padding_byte_count: usize,
    ) -> std::io::Result<()> {
        let input = File::open(input_filename)?;
        let output = File::create(output_filename)?;
        let mut reader = BufReader::new(input);
        let mut writer = BufWriter::new(output);

        let reversed_table = Codec::decode_header(&mut reader)?;
        Codec::decode_body(&mut reader, &mut writer, reversed_table, padding_byte_count)
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
        padding_byte_count: usize,
    ) -> std::io::Result<()> {
        let mut key = String::new();
        let mut buffer = [0; BUFFER_SIZE];
        let mut total_bits_read = 0; // Track the total number of bits read

        loop {
            let limit = reader.read(&mut buffer)?;
            if limit == 0 {
                break; // End of file
            }

            for (i, byte) in buffer.iter().take(limit).enumerate() {
                // Determine how many bits to read from the current byte
                let bits_to_read = if i == limit - 1 {
                    // Last byte, adjust for padding
                    8 - padding_byte_count // Read only valid bits in the last byte
                } else {
                    8 // Read all bits in other bytes
                };

                for bit_pos in 0..bits_to_read {
                    let bit = (byte >> (7 - bit_pos)) & 1; // Extract each bit
                    key.push(if bit == 1 { '1' } else { '0' }); // Append to key

                    // Check if we have a complete character in the reversed table
                    if let Some(&ch) = reversed_table.get(&key) {
                        // Encode character as a UTF-8 sequence and write to output
                        let mut buf = [0; 4];
                        let ch_utf8 = ch.encode_utf8(&mut buf);
                        writer.write_all(ch_utf8.as_bytes())?; // Write the encoded character
                        key.clear(); // Reset key after writing
                    }
                }
                total_bits_read += bits_to_read; // Increment total bits read
            }
        }

        writer.flush()?;
        Ok(())
    }
}
