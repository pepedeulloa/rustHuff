mod cli;
use crate::cli::*;

mod huff;
use crate::huff::*;

mod calc;
use crate::calc::*;

mod header;
use crate::header::*;

mod encoder; 
use crate::encoder::*;

mod decoder;
use crate::decoder::*;

use std::io::Read;
use std::sync::Arc;
use std::time::Instant;

fn main() {
	// Parsing cli args
	let Cli{file, output, decode} = get_args().unwrap();

	if !decode {
		// Read the file and parse to a string 
	let mut reader = open_file(&file).unwrap();
	let mut file_str = String::new();

	reader.read_to_string(&mut file_str).unwrap();

	// Calculate the frequencies
	let frequencies: Vec<_> = calculate_frequencies(&file_str).unwrap();
	let mut huff_vector = HuffVector::gen_huff_vector(frequencies);
	
	// Create the Huffman tree with the frequencies.
	let huff_tree = HuffBTree::gen_tree(&mut huff_vector);

	// Genenate the table with the codes.
	let table = Arc::new(huff_tree.gen_table());

	// Generate the data to write
	let header: (usize, String, Vec<(Vec<u8>, usize, Vec<bool>)>) = get_headers(file.extension().unwrap().to_str().unwrap() ,table.as_ref());
	let start_encoding = Instant::now();
	let encoded_data = encode_mt(table, file_str);
	let end_encoding = Instant::now();
	let encoding_time = end_encoding - start_encoding;
	println!("Tempo de codificación: {}ms", encoding_time.as_millis());
	
	// Write the file
	let start_write = Instant::now();
	write_encoded_file(&output.unwrap(), header, encoded_data).unwrap();
	let end_write = Instant::now();
	let writing_time = end_write - start_write;
	println!("Tempo de escritura: {}ms", writing_time.as_millis());

	} else {
		let mut encoded_reader = open_file(&file).unwrap();

		let (_, extension, table) = parse_headers(&mut encoded_reader);

		let table = Arc::new(table);
		
		let start_decoding_time = Instant::now();
		let decoded_text = decode_file(&mut encoded_reader, table);
		let end_decoding_time = Instant::now();
		let decoding_time = end_decoding_time - start_decoding_time;

		println!("Tempo de decodificación: {}ms", decoding_time.as_millis());

		let start_write = Instant::now();
		write_decoded_file(&output.unwrap().with_extension(extension), decoded_text).unwrap();		
		let end_write = Instant::now();
		let writing_time = end_write - start_write;
		println!("Tempo de escritura: {}ms", writing_time.as_millis());
	
	}
}